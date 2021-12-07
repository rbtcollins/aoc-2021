use std::str;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Default, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Default, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Default, Clone, Debug)]
struct Floor {
    floor: Vec<Vec<usize>>,
}

#[derive(Default, Clone, Debug)]
struct Game {
    lines: Vec<Line>,
}

#[aoc_generator(day5)]
fn generate(input: &str) -> Game {
    Game {
        lines: input
            .lines()
            .flat_map(|s| s.split(" -> "))
            .flat_map(|s| s.split(','))
            .flat_map(|s| s.parse())
            .chunks(2)
            .into_iter()
            .map(|mut scalars| Point {
                x: scalars.next().unwrap(),
                y: scalars.next().unwrap(),
            })
            .chunks(2)
            .into_iter()
            .map(|mut points| Line {
                start: points.next().unwrap(),
                end: points.next().unwrap(),
            })
            .collect(),
    }
}

impl Line {
    fn axis_aligned(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

impl Floor {
    fn size_from<'a, V: Iterator<Item = &'a Line>>(vents: V) -> usize {
        let mut max = 0;
        for line in vents {
            max = std::cmp::max(max, line.start.x);
            max = std::cmp::max(max, line.start.y);
            max = std::cmp::max(max, line.end.x);
            max = std::cmp::max(max, line.end.y);
        }
        max + 1 // index 9 => 10 positions
    }

    fn new(size: usize) -> Self {
        let mut res = Floor::default();
        for _ in 0..size {
            let row = vec![0; size];
            res.floor.push(row);
        }
        res
    }

    fn render<'a, V: Iterator<Item = &'a Line>>(&mut self, vents: V) {
        for line in vents {
            // 45' diagonal
            // 3,1 -> -2
            let width_x: isize = line.end.x as isize - line.start.x as isize;
            // -2 -> 2
            let steps_x = width_x.abs();
            // -2/2 = -1
            let delta_x = if steps_x == 0 { 0 } else { width_x / steps_x };
            let width_y: isize = line.end.y as isize - line.start.y as isize;
            let steps_y = width_y.abs();
            let delta_y = if steps_y == 0 { 0 } else { width_y / steps_y };
            let steps = if steps_x == 0 { steps_y } else { steps_x };
            let mut x = line.start.x;
            let mut y = line.start.y;
            for _ in 0..steps + 1 {
                *self.cell(x, y) += 1;
                x = (x as isize + delta_x) as usize;
                y = (y as isize + delta_y) as usize;
            }
        }
    }

    fn cell(&mut self, x: usize, y: usize) -> &mut usize {
        &mut self.floor[x][y]
    }

    fn cells(&self) -> impl Iterator<Item = usize> + '_ {
        self.floor.iter().flat_map(|r| r.iter()).copied()
    }
}

#[aoc(day5, part1)]
fn part1(input: &Game) -> usize {
    let (vents1, vents2) = input.lines.iter().filter(|l| l.axis_aligned()).tee();
    let mut floor = Floor::new(Floor::size_from(vents1));
    floor.render(vents2);
    floor.cells().filter(|cell| cell >> 1 > 0).count()
}

#[aoc(day5, part2)]
fn part2(input: &Game) -> usize {
    let mut floor = Floor::new(Floor::size_from(input.lines.iter()));
    floor.render(input.lines.iter());
    floor.cells().filter(|cell| cell >> 1 > 0).count()
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

    #[test]
    fn part1() {
        let input = super::generate(SAMPLE);
        assert_eq!(5, super::part1(&input));
    }

    #[test]
    fn part2() {
        let input = super::generate(SAMPLE);
        assert_eq!(12, super::part2(&input));
    }
}
