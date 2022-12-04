use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn generate(input: &str) -> String {
    input.to_owned()
}

fn fully_contains(r1: &[usize], r2: &[usize]) -> bool {
    r1[0] <= r2[0] && r1[1] >= r2[1]
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|p| {
                    p.split('-')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>()
        })
        .filter(|l| fully_contains(&l[0], &l[1]) || fully_contains(&l[1], &l[0]))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|p| {
                    p.split('-')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>()
        })
        .filter(|l| !((l[0][1] < l[1][0]) || (l[0][0] > l[1][1])))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // assert_eq!(part2(&generate("2-4,6-8")), 0);
        // assert_eq!(part2(&generate("2-3,4-5",)), 0);
        // assert_eq!(part2(&generate("5-7,7-9",)), 1);
        // assert_eq!(part2(&generate("2-8,3-7",)), 1);
        // assert_eq!(part2(&generate("6-6,4-6",)), 1);
        // assert_eq!(part2(&generate("2-6,4-8",)), 1);
        assert_eq!(
            part1(&generate(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
            )),
            2
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generate("2-4,6-8")), 0);
        assert_eq!(part2(&generate("2-3,4-5",)), 0);
        assert_eq!(part2(&generate("5-7,7-9",)), 1);
        assert_eq!(part2(&generate("2-8,3-7",)), 1);
        assert_eq!(part2(&generate("6-6,4-6",)), 1);
        assert_eq!(part2(&generate("2-6,4-8",)), 1);
        assert_eq!(
            part2(&generate(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
            )),
            4
        );
    }
}
