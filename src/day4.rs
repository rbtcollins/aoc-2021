use std::ops::Mul;

use rayon::iter::{IntoParallelIterator, ParallelIterator as _};

pub fn generate(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect::<Vec<_>>()
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn checked_add(&self, vector: Vector) -> Option<Point> {
        let x = self.x as isize + vector.dx;
        let y = self.y as isize + vector.dy;
        if x < 0 || y < 0 {
            None
        } else {
            Some(Point {
                x: x as usize,
                y: y as usize,
            })
        }
    }

    fn checked_sub(&self, vector: Vector) -> Option<Point> {
        let x = self.x as isize - vector.dx;
        let y = self.y as isize - vector.dy;
        if x < 0 || y < 0 {
            None
        } else {
            Some(Point {
                x: x as usize,
                y: y as usize,
            })
        }
    }

    fn within(&self, max: &Point) -> Option<Point> {
        if self.x < max.x && self.y < max.y {
            Some(*self)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
struct Vector {
    dx: isize,
    dy: isize,
}

impl Mul<usize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: usize) -> Vector {
        Vector {
            dx: self.dx * rhs as isize,
            dy: self.dy * rhs as isize,
        }
    }
}

fn xmas_at_vector(input: &[&[u8]], start: Point, delta: Vector, max: &Point) -> usize {
    [b'X', b'M', b'A', b'S']
        .iter()
        .try_fold(start, |p, &ch| {
            if input[p.y][p.x] != ch {
                return None;
            }
            p.checked_add(delta).and_then(|p| p.within(max))
        })
        .map(|_| 1)
        .unwrap_or(0)
}

fn xmas_at(input: &[&[u8]], p: Point, max: &Point) -> usize {
    [
        Vector { dx: 1, dy: 0 },   // →
        Vector { dx: 1, dy: 1 },   // ↘
        Vector { dx: 0, dy: 1 },   // ↓
        Vector { dx: -1, dy: 1 },  // ↙
        Vector { dx: -1, dy: 0 },  // ←
        Vector { dx: -1, dy: -1 }, // ↖
        Vector { dx: 0, dy: -1 },  // ↑
        Vector { dx: 1, dy: -1 },  // ↗
    ]
    .iter()
    .map(|&delta| xmas_at_vector(input, p, delta, max))
    .sum()
}

/// Find the word XMAS orientated in horizontal or diagonal direction
pub fn part_1(input: &[&[u8]]) -> usize {
    let max_x = input[0].len() as isize;
    let max_y = input.len() as isize;
    let max = Point {
        x: max_x as usize,
        y: max_y as usize,
    };

    (0..input[0].len())
        .map(|x| -> usize {
            (0..input.len())
                .map(|y| xmas_at(&input, Point { x, y }, &max))
                .sum()
        })
        .sum()
}

pub fn part_1_rayon(input: &[&[u8]]) -> usize {
    let max_x = input[0].len() as isize;
    let max_y = input.len() as isize;
    let max = Point {
        x: max_x as usize,
        y: max_y as usize,
    };

    (0..input[0].len())
        .into_par_iter()
        .map(|x| -> usize {
            (0..input.len())
                .map(|y| xmas_at(&input, Point { x, y }, &max))
                .sum()
        })
        .sum()
}

fn ms_at_vector(input: &[&[u8]], p: Point, delta: Vector) -> bool {
    const MS: u8 = b'M' | b'S';

    let first = p.checked_sub(delta);
    let second = p.checked_add(delta);
    let (Some(first), Some(second)) = (first, second) else {
        return true;
    };
    input[first.y][first.x] | input[second.y][second.x] == MS
}

fn x_mas_at(input: &[&[u8]], p: Point) -> usize {
    if input[p.y][p.x] != b'A' {
        return 0;
    }
    [
        Vector { dx: 1, dy: 1 },  // ↘
        Vector { dx: -1, dy: 1 }, // ↙
    ]
    .iter()
    .map(|&delta| ms_at_vector(input, p, delta))
    .all(|v| v) as usize
}

/// Find an A with MS across each diagonal in either direction
pub fn part_2(input: &Vec<&[u8]>) -> usize {
    (1..input[0].len() - 1)
        .map(|x| -> usize {
            (1..input.len() - 1)
                .map(|y| x_mas_at(&input, Point { x, y }))
                .sum()
        })
        .sum()
}

pub fn part_2_rayon(input: &Vec<&[u8]>) -> usize {
    (1..input[0].len() - 1)
        .into_par_iter()
        .map(|x| -> usize {
            (1..input.len() - 1)
                .map(|y| x_mas_at(&input, Point { x, y }))
                .sum()
        })
        .sum()
}

#[cfg(test)]
mod tests {}
