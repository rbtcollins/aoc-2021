use std::str;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default, Clone, Debug)]
struct Move {
    number: usize,
    from: usize,
    to: usize,
}

#[derive(Default, Clone, Debug)]
struct Plan {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

#[aoc_generator(day5)]
fn generate(input: &str) -> Plan {
    let mut input = input.lines();
    let mut stacks_matrix = input
        .by_ref()
        .take_while(|l| !l.starts_with(" 1"))
        .map(|l| {
            l.as_bytes()
                .chunks(4)
                .map(|c| {
                    if c[1] == b' ' {
                        None
                    } else {
                        Some(char::from(c[1]))
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    stacks_matrix.reverse();
    let len = stacks_matrix[0].len();
    let stacks = (-1..len as i8)
        .into_iter()
        .map(|i| {
            if i >= 0 {
                // a single empty stack at offset 0.
                stacks_matrix
                    .iter()
                    .filter_map(|row| row[i as usize])
                    .collect()
            } else {
                vec![]
            }
        })
        .collect::<Vec<Vec<_>>>();
    // skip the index line
    input.next();
    // create moves vector
    // move 6 from 6 to 5
    let moves = input
        .filter(|l| !l.is_empty())
        .map(|l| {
            let l = l.split(' ').collect::<Vec<_>>();
            Move {
                number: l[1].parse().unwrap(),
                from: l[3].parse().unwrap(),
                to: l[5].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    Plan { stacks, moves }
}

#[aoc(day5, part1)]
fn part1(input: &Plan) -> String {
    let mut stacks = input.stacks.clone();
    for crate_move in &input.moves {
        for _ in 0..crate_move.number {
            let item = stacks[crate_move.from].pop().unwrap();
            stacks[crate_move.to].push(item);
        }
    }
    stacks.iter().skip(1).map(|s| s.last().unwrap()).collect()
}

#[aoc(day5, part2)]
fn part2(input: &Plan) -> String {
    let mut stacks = input.stacks.clone();
    for crate_move in &input.moves {
        let from = &mut stacks[crate_move.from];
        let mut items = from.split_off(from.len() - crate_move.number);
        stacks[crate_move.to].append(&mut items);
    }
    stacks.iter().skip(1).map(|s| s.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn part1() {
        let input = super::generate(SAMPLE);
        assert_eq!("CMZ", super::part1(&input));
    }

    #[test]
    fn part2() {
        let input = super::generate(SAMPLE);
        assert_eq!("MCD", super::part2(&input));
    }
}
