use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{EitherOrBoth, Itertools};

#[aoc_generator(day3)]
fn generate(input: &str) -> Vec<Vec<Vec<u8>>> {
    input
        .lines()
        .map(|l| {
            l.as_bytes()
                .chunks(l.len() >> 1)
                .map(|c| {
                    c.iter()
                        .map(|b| {
                            if *b > b'Z' {
                                b - b'a' + 1 // a -> 1
                            } else {
                                b - b'A' + 27 // A -> 27
                            }
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<Vec<u8>>]) -> usize {
    input
        .iter()
        .map(|rs| {
            rs[0]
                .iter()
                .sorted()
                .merge_join_by(rs[1].iter().sorted(), |i, j| (*i).cmp(*j))
                .filter_map(|m| match m {
                    EitherOrBoth::Both(i, _) => Some(*i as usize),
                    _ => None,
                })
                .unique()
                .sum::<usize>()
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<Vec<u8>>]) -> usize {
    input
        .chunks_exact(3)
        .map(|c| {
            c[0][0]
                .iter()
                .chain(c[0][1].iter())
                .sorted()
                .merge_join_by(c[1][0].iter().chain(c[1][1].iter()).sorted(), |i, j| {
                    (*i).cmp(*j)
                })
                .filter_map(|m| match m {
                    EitherOrBoth::Both(i, _) => Some(*i),
                    _ => None,
                })
                .merge_join_by(c[2][0].iter().chain(c[2][1].iter()).sorted(), |i, j| {
                    (*i).cmp(*j)
                })
                .filter_map(|m| match m {
                    EitherOrBoth::Both(i, _) => Some(i as usize),
                    _ => None,
                })
                .unique()
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&generate(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
            )),
            157
        );
    }

    #[test]
    fn test_part1_individual() {
        assert_eq!(part1(&generate("vJrwpWtwJgWrhcsFMMfFFhFp",)), 16);
        assert_eq!(part1(&generate("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",)), 38);
        assert_eq!(part1(&generate("PmmdzqPrVvPwwTWBwg",)), 42);
        assert_eq!(part1(&generate("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",)), 22);
        assert_eq!(part1(&generate("ttgJtRGJQctTZtZT",)), 20);
        assert_eq!(part1(&generate("CrZsJsPPZsGzwwsLwLmpwMDw",)), 19);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&generate(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
            )),
            70
        );
    }
}
