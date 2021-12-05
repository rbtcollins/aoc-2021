use aoc_runner_derive::{aoc, aoc_generator};

#[allow(non_camel_case_types)]
#[derive(enum_utils::FromStr)]
enum Direction {
    forward,
    down,
    up,
}

#[aoc_generator(day2)]
fn generate(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(Direction, usize)]) -> usize {
    let mut depth = 0;
    let mut distance = 0;
    for (direction, amount) in input.into_iter() {
        match direction {
            Direction::forward => distance += amount,
            Direction::up => depth -= amount,
            Direction::down => depth += amount,
        };
    }
    depth * distance
}

#[aoc(day2, part2)]
fn part2(input: &[(Direction, usize)]) -> usize {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    for (direction, amount) in input.into_iter() {
        match direction {
            Direction::forward => {
                distance += amount;
                depth += aim * amount
            }
            Direction::up => aim -= amount,
            Direction::down => aim += amount,
        };
    }
    depth * distance
}
