use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

#[aoc_generator(day1)]
fn generate(input: &[u8]) -> Vec<u32> {
    let mut result = Vec::<u32>::new();
    let mut current: u32 = 0;
    for byte in input.iter() {
        if byte < &b'0' || byte > &b'9' {
            result.push(current);
            current = 0;
            continue;
        }
        let byte_value: u32 = (byte - b'0').into();
        if current != 0 {
            current = current * 10 + byte_value;
        } else {
            current = byte_value;
        }
    }
    if current != 0 {
        result.push(current);
    }
    result
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> usize {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

#[aoc(day1, part1, rayon)]
fn part1_rayon(input: &[u32]) -> usize {
    input
        .par_windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> usize {
    input
        .windows(4)
        .filter(|window| window[0] < window[3])
        .count()
}

#[aoc(day1, part2, rev)]
fn part2_for(input: &[u32]) -> usize {
    let mut count = 0;
    for pos in 3..input.len() {
        if input[pos] > input[pos - 3] {
            count = count + 1;
        }
    }
    count
}
