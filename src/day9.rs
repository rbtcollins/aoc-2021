use std::slice::Iter;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lending_iterator::prelude::*;

#[aoc_generator(day9)]
fn generate(input: &str) -> Vec<(Direction, usize)> {
    input
        .split(&[' ', '\n'][..])
        .tuples()
        .map(|(d, size)| {
            let size = str::parse::<usize>(size).unwrap();
            (
                match d {
                    "D" => Direction::D,
                    "L" => Direction::L,
                    "R" => Direction::R,
                    "U" => Direction::U,
                    _x => unreachable!("{_x}"),
                },
                size,
            )
        })
        .collect()
}

/// input commands
#[derive(Debug, Clone, Copy)]
enum Direction {
    R,
    D,
    L,
    U,
}

struct RopeMoves<'a> {
    iter: Iter<'a, (Direction, usize)>,
    segments: Vec<(isize, isize)>,

    // built in flatten
    current_direction: Direction,
    remaining_distance: usize,
    //
    last_segment: usize,
}

impl<'a> RopeMoves<'a> {
    fn new(moves: &'a [(Direction, usize)], length: usize) -> Self {
        RopeMoves {
            iter: moves.iter(),
            segments: vec![(0, 0); length],
            current_direction: Direction::U,
            remaining_distance: 0,
            last_segment: length - 1,
        }
    }
}

impl Iterator for RopeMoves<'_> {
    type Item = (isize, isize);

    // yields the tail position after applying the move
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_distance == 0 {
            let (current_direction, remaining_distance) = self.iter.next()?;
            self.current_direction = *current_direction;
            self.remaining_distance = *remaining_distance;
        }

        match self.current_direction {
            Direction::U => self.segments[0].1 += 1,
            Direction::D => self.segments[0].1 -= 1,
            Direction::L => self.segments[0].0 -= 1,
            Direction::R => self.segments[0].0 += 1,
        }
        // process each segment bringing them into compliance.
        let mut iter = self.segments.windows_mut::<2>();
        while let Some(&mut [h, ref mut t]) = iter.next() {
            match (h.0 - t.0, h.1 - t.1) {
                (1, 0)
                | (1, -1)
                | (0, -1)
                | (-1, -1)
                | (-1, 0)
                | (-1, 1)
                | (0, 1)
                | (1, 1)
                | (0, 0) => break,
                // corners - distances of 2
                (2, 2) => {
                    t.0 += 1;
                    t.1 += 1
                }
                (2, -2) => {
                    t.0 += 1;
                    t.1 -= 1
                }
                (-2, -2) => {
                    t.0 -= 1;
                    t.1 -= 1
                }
                (-2, 2) => {
                    t.0 -= 1;
                    t.1 += 1
                }
                // edge and knight moves - one step and up to one orthogonally
                (2, d) => {
                    t.0 += 1;
                    t.1 += d
                }
                (d, -2) => {
                    t.0 += d;
                    t.1 -= 1
                }
                (-2, d) => {
                    t.0 -= 1;
                    t.1 += d
                }
                (d, 2) => {
                    t.0 += d;
                    t.1 += 1
                }
                _v => unreachable!("{_v:?}"),
            }
        }
        self.remaining_distance -= 1;
        Some(self.segments[self.last_segment])
    }
}

#[aoc(day9, part1)]
fn part1(input: &[(Direction, usize)]) -> usize {
    RopeMoves::new(input, 2).unique().count()
}

#[aoc(day9, part2)]
fn part2(input: &[(Direction, usize)]) -> usize {
    RopeMoves::new(input, 10).unique().count()
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    const SAMPLE_2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn part1() {
        let input = super::generate(SAMPLE);
        assert_eq!(13, super::part1(&input));
    }

    #[test]
    fn part2_a() {
        let input = super::generate(SAMPLE);
        assert_eq!(1, super::part2(&input));
    }

    #[test]
    fn part2_b() {
        let input = super::generate(SAMPLE_2);
        assert_eq!(36, super::part2(&input));
    }
}
