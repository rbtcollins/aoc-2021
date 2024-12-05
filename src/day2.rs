use winnow::combinator::{opt, terminated};
use winnow::{ascii::space0, combinator::repeat};
use winnow::{
    ascii::{dec_int, line_ending},
    error::ContextError,
    prelude::*,
};

pub fn generate(input: &str) -> Vec<Vec<i32>> {
    repeat(
        0..,
        terminated::<_, Vec<i32>, _, _, _, _>(
            repeat(1.., terminated(dec_int::<_, i32, ContextError>, space0)),
            opt(line_ending),
        ),
    )
    .parse(input)
    .unwrap()
}

pub fn part_1(input: &[Vec<i32>]) -> usize {
    // count the reports which are monotonic in any direction with step sizes between 1 and 3
    input
        .iter()
        .filter(|report| {
            report
                .windows(2)
                .try_fold(0, |dir, pair| fold_pair(pair[0], pair[1], dir))
                .is_ok()
        })
        .count()
}

fn fold_pair(l: i32, r: i32, dir: i32) -> Result<i32, ()> {
    let direction = match r - l {
        ..=-4 | 4.. => return Err(()),
        ..=-1 => 2 | dir,
        1.. => 1 | dir,
        _ => return Err(()),
    };
    match direction | dir {
        dir @ 1 | dir @ 2 => Ok(dir),
        _ => Err(()),
    }
}

/// NOTETO SELF:
/// if the step is too large,  (1,7,8) recovery means taking the prior value and checking the step again
///  - we detect large steps on the second element
/// if the sign was wrong, (3,2,3) recovery means taking the next value
///  - we detect the sign error on the first element
pub fn part_2(input: &[Vec<i32>]) -> usize {
    // as part 1, but tolerate a single bad step
    if true {
        input
            .iter()
            .filter(|&report| {
                (0..=report.len())
                    .filter(|&i| {
                        let mut report = report.clone();
                        if i > 0 {
                            report.remove(i - 1);
                        }
                        let succeeds = report
                            .windows(2)
                            .try_fold(0, |dir, pair| fold_pair(pair[0], pair[1], dir))
                            .is_ok();
                        // eprintln!("{:?} -> {}", report, succeeds);
                        succeeds
                    })
                    .take(1)
                    .count()
                    > 0
            })
            .count()
    } else {
        // single pass version WIP
        input
            .iter()
            .filter(|&report| {
                fn validate_report(report: &Vec<i32>, error_location: Option<usize>) -> bool {
                    dbg!(report);
                    report
                        .windows(2)
                        .enumerate()
                        .try_fold(
                            (0, None, error_location),
                            |(direction, alternate_left, error_location), (i, pair)| {
                                dbg!((direction, alternate_left, error_location, pair));
                                let (new_error_location, folded) = match error_location {
                                    // If an error has occured on the previous step, (e.g. 1->7 from 1 7 8) then
                                    // going from previous to next is the only valid attempt
                                    Some(loc) if loc + 1 == i => (
                                        error_location,
                                        fold_pair(
                                            alternate_left.unwrap_or(pair[0]),
                                            pair[1],
                                            direction,
                                        ),
                                    ),
                                    // If an error has occured earlier, then we can only try the current pair
                                    Some(_) => {
                                        (error_location, fold_pair(pair[0], pair[1], direction))
                                    }
                                    // Try this pair two ways: once as given, and only if that fails, once using the previous element as the left (if it exists).
                                    _ => {
                                        let current = fold_pair(pair[0], pair[1], direction);

                                        if current.is_ok() {
                                            (None, current)
                                        } else {
                                            // 1 2 3 -> [01, 12] : last interval is 1 when length is three
                                            // intervals, not indexes
                                            dbg!((i, report.len()));
                                            if i + 2 == report.len() {
                                                // no current error at the end, just skip the last element.
                                                (None, Ok(direction))
                                            } else if alternate_left.is_some() {
                                                let alternate = fold_pair(
                                                    alternate_left.unwrap_or(pair[0]),
                                                    pair[1],
                                                    direction,
                                                );
                                                (Some(i), alternate)
                                            } else {
                                                // error on the first step
                                                (Some(i), current)
                                            }
                                        }
                                    } // fold_pair(pair[0], pair[1], direction).or_else(|_| {
                                      //     fold_pair(alternate_left.unwrap_or(pair[0]), pair[1], direction)
                                      // }),
                                };
                                // let folded = fold_pair(pair[0], pair[1], direction).or_else(|_| {
                                //     fold_pair(alternate_left.unwrap_or(pair[0]), pair[1], direction)
                                // });
                                if let Ok(new_direction) = folded {
                                    return Ok((new_direction, Some(pair[0]), new_error_location));
                                } else if error_location.is_some() {
                                    // second error
                                    return Err(());
                                } else {
                                    // record the error, and try again with the next pair.
                                    return Ok((direction, Some(pair[0]), new_error_location));
                                }
                            },
                        )
                        .is_ok()
                }

                // edge case: cannot infer fault location correctly if the first element is the faulty one; try skipping it entirely and re-run.
                validate_report(report, None) || {
                    validate_report(&report.iter().copied().skip(1).collect(), Some(0))
                }
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_records() {
        for row in [
            ("7 6 4 2 1", 1, 1),
            ("1 2 7 8 9", 0, 0),
            ("9 7 6 2 1", 0, 0),
            ("1 3 2 4 5", 0, 1), // skip 3
            ("8 6 4 4 1", 0, 1), // skip 4
            ("1 3 6 7 9", 1, 1),
            ("1 5 6", 0, 1),     // skip 1, it works
            ("3 1 2 4 5", 0, 1), // skip 3
            ("2 3 4 5 2", 0, 1), // skip second 2
            ("1 10 4", 0, 1),    // skip 10, it works.
            ("1 4 10", 0, 1),    // skip 10
            ("1 3 2 5 4", 0, 0),
            ("0 3 2 5 6", 0, 1),
            ("0 3 2 3 4", 0, 1), // skip first 3
            ("2 3 2 3 4", 0, 0),
        ] {
            let input = generate(row.0);
            assert_eq!(part_1(&input), row.1, "{:?}", row.0);
            assert_eq!(part_2(&input), row.2, "{:?}", row.0);
        }
    }

    #[derive(Debug, Clone)]
    enum Outcomes {
        Failure,
        SuccessWithoutErrors,
        SuccessWithErrors,
    }

    #[derive(Debug, Clone)]
    enum LevelError {
        Direction,
        Size(usize),
    }

    #[derive(Debug, Clone)]
    struct BadLevel {
        interval: usize,
        kind: LevelError,
    }

    fn arb_bad_level() -> impl Strategy<Value = BadLevel> {
        let level_error = prop_oneof![
            Just(LevelError::Direction),
            // size of 0 is an invalid step, as is a too-large step.
            prop_oneof![Just(0usize), 4usize..10].prop_map(LevelError::Size)
        ];
        (0usize..4, level_error).prop_map(|(interval, kind)| dbg!(BadLevel { interval, kind }))
    }

    fn apply_error(v: &mut Vec<i32>, bad: &BadLevel, direction: i32) {
        assert_ne!(direction, 0);
        match bad.kind {
            LevelError::Direction => {
                let existing_step = v[bad.interval + 1] - v[bad.interval];
                let delta = -existing_step - direction;
                for i in (bad.interval + 1)..v.len() {
                    v[i] = v[i] + delta;
                }
            }
            LevelError::Size(size) => {
                // calulate the change needed to make the interval the given size
                eprintln!("size: {size}, {v:?}, {bad:?}");
                let delta = (direction * size as i32) - (v[bad.interval + 1] - v[bad.interval]);
                // rewrite the array from the index to the end preserving the existing deltas.
                for i in (bad.interval + 1)..v.len() {
                    v[i] = v[i] + delta;
                }
            }
        }
    }

    fn arb_record() -> impl Strategy<Value = (Vec<i32>, usize, usize, BadLevel, BadLevel, i32)> {
        (
            prop_oneof![Just(1), Just(-1),],
            0i32..100,
            prop_oneof![
                Just(Outcomes::Failure),
                Just(Outcomes::SuccessWithoutErrors),
                Just(Outcomes::SuccessWithErrors)
            ],
            // single-error
            arb_bad_level(),
            // second error offset (0..3 + 1..4 % 4 is unique)
            1usize..4,
            // second error details
            arb_bad_level(),
        )
            .prop_map(
                |(direction, start, outcome, single_error, second_offset, mut second_error)| {
                    second_error.interval = (single_error.interval + second_offset) % 5;
                    (direction, start, outcome, single_error, second_error)
                },
            )
            .prop_map(|(direction, start, outcome, single_error, second_error)| {
                assert_ne!(single_error.interval, second_error.interval);
                let mut v = vec![start; 5];
                for i in 1..v.len() {
                    v[i] = v[i - 1] + 2 * direction;
                }
                match outcome {
                    Outcomes::SuccessWithoutErrors => {}
                    Outcomes::SuccessWithErrors => {
                        apply_error(&mut v, &single_error, direction);
                    }
                    Outcomes::Failure => {
                        apply_error(&mut v, &single_error, direction);
                        apply_error(&mut v, &second_error, direction);
                    }
                }
                let &min = v.iter().min().unwrap();
                if min < 0 {
                    for i in 0..v.len() {
                        v[i] = v[i] - min;
                    }
                }
                for i in 0..v.len() {
                    if v[i] < 0 {
                        v[i] = 0;
                    }
                }
                dbg!((
                    v,
                    match outcome {
                        Outcomes::SuccessWithoutErrors => 1,
                        _ => 0,
                    },
                    match outcome {
                        Outcomes::Failure => 0,
                        _ => 1,
                    },
                    single_error,
                    second_error,
                    direction
                ))
            })
    }

    proptest! {

        #[test]
        #[ignore]
        fn test_arb_records(i in arb_record()) {
            let (input, success, success_with_skip, _err_1, _err_2, _direction) = i;
            let input = vec![input];
            prop_assert_eq!(part_1(&input), success, "{:?}", input);
            prop_assert_eq!(part_2(&input), success_with_skip, "{:?}", input);
        }
    }

    #[test]
    fn test_apply_error_size_0() {
        let mut v = vec![0, 2, 4, 6, 8];
        apply_error(
            &mut v,
            &BadLevel {
                interval: 0,
                kind: LevelError::Size(0),
            },
            1,
        );
        assert_eq!(v, vec![0, 0, 2, 4, 6]);
    }

    #[test]
    fn test_apply_error_direction() {
        // a change in direction preserves the steps before and after the mutated interval.
        let mut v = vec![0, 0, 2, 2];
        apply_error(
            &mut v,
            &BadLevel {
                interval: 1,
                kind: LevelError::Direction,
            },
            1,
        );
        assert_eq!(v, vec![0, 0, -1, -1]);
    }

    #[test]
    fn test_apply_error_2() {
        let mut v = vec![0, 2, 4, 6, 8];
        apply_error(
            &mut v,
            &BadLevel {
                interval: 1,
                kind: LevelError::Size(0),
            },
            1,
        );
        assert_eq!(v, vec![0, 2, 2, 4, 6]);
        apply_error(
            &mut v,
            &BadLevel {
                interval: 0,
                kind: LevelError::Direction,
            },
            1,
        );
        assert_eq!(v, vec![0, -1, -1, 1, 3]);
    }
}
