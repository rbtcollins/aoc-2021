use std::collections::HashSet;
use std::str;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Default, Debug, Clone)]
struct Board {
    rows: Vec<Vec<usize>>,
}

#[derive(Default, Clone)]
struct Game {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

#[aoc_generator(day4)]
fn generate(input: &str) -> Game {
    Game {
        numbers: input
            .lines()
            .take(1)
            .flat_map(|l| l.split(','))
            .flat_map(|s| s.parse())
            .collect(),
        boards: input
            .lines()
            .skip(1)
            .chunks(6)
            .into_iter()
            .flat_map(|lines| lines.skip(1))
            .map(|l| l.split(' ').flat_map(|s| s.parse()).collect::<Vec<usize>>())
            .chunks(5)
            .into_iter()
            .map(|rows| Board {
                rows: rows.collect::<Vec<Vec<usize>>>(),
            })
            .collect(),
    }
}

impl Game {
    fn winning_board(&self, drawn: &HashSet<usize>) -> Option<(usize, &Board)> {
        for (index, board) in self.boards.iter().enumerate() {
            if board.won(drawn) {
                return Some((index, board));
            }
        }
        None
    }
}

impl Board {
    fn won(&self, drawn: &HashSet<usize>) -> bool {
        for row in self.rows.iter().cloned().chain(self.columns()) {
            if row.iter().all(|n| drawn.contains(n)) {
                return true;
            }
        }
        false
    }
    fn column(&self, n: usize) -> impl Iterator<Item = usize> + '_ {
        self.rows
            .iter()
            .flat_map(|r| r.iter())
            .skip(n)
            .step_by(5)
            .copied()
    }
    fn columns(&self) -> impl Iterator<Item = Vec<usize>> + '_ {
        (0..5).map(|i| self.column(i).collect::<Vec<_>>())
    }
    fn unmarked<'a>(&'a self, drawn: &'a HashSet<usize>) -> impl Iterator<Item = &'a usize> {
        self.rows
            .iter()
            .flat_map(|r| r.iter())
            .filter(|n| !drawn.contains(n))
    }
}

#[aoc(day4, part1)]
fn part1(input: &Game) -> usize {
    let mut drawn: HashSet<usize> = HashSet::new();
    for number in &input.numbers {
        drawn.insert(*number);
        if let Some((_, board)) = input.winning_board(&drawn) {
            let unmarked: usize = board.unmarked(&drawn).sum();
            return unmarked * number;
        }
    }
    panic!("Unreachable");
}

#[aoc(day4, part2)]
fn part2(input: &Game) -> usize {
    let mut input = input.clone();
    let mut drawn: HashSet<usize> = HashSet::new();
    for number in &input.numbers {
        drawn.insert(*number);
        while let Some((index, board)) = input.winning_board(&drawn) {
            if input.boards.len() == 1 {
                let unmarked: usize = board.unmarked(&drawn).sum();
                return unmarked * number;
            } else {
                input.boards.remove(index);
            }
        }
    }
    panic!("Unreachable");
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    #[test]
    fn part1() {
        let input = super::generate(SAMPLE);
        assert_eq!(4512, super::part1(&input));
    }

    #[test]
    fn part2() {
        let input = super::generate(SAMPLE);
        assert_eq!(1924, super::part2(&input));
    }
}
