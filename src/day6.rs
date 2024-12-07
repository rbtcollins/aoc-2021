use core::str;
use std::{
    collections::VecDeque,
    fmt::Debug,
    ops::{Add, Index, IndexMut},
};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};

const BLOCKED: u8 = b'#';
const VISITED: u8 = b'X';
// const EMPTY: u8 = b'.';
const START: u8 = b'^';
const EDGE: u8 = b'E';

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn within(&self, max: &Point) -> Option<Point> {
        if self.x < max.x && self.y < max.y {
            Some(*self)
        } else {
            None
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Point {
        Point {
            x: x as isize,
            y: y as isize,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vector {
    dx: isize,
    dy: isize,
}

#[derive(Clone)]
pub struct Puzzle {
    map: Vec<Vec<u8>>,
    pos: Point,
    max: Point,
    zero: Point,
}

impl Index<Point> for Puzzle {
    type Output = u8;

    fn index(&self, index: Point) -> &Self::Output {
        &self.map[index.y as usize][index.x as usize]
    }
}

impl IndexMut<Point> for Puzzle {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.map[index.y as usize][index.x as usize]
    }
}

impl Debug for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        eprintln!("");
        for row in &self.map {
            for cell in row {
                write!(f, "{}", *cell as char)?;
            }
            writeln!(f)?;
        }
        f.debug_struct("Puzzle")
            .field("pos", &self.pos)
            .field("max", &self.max)
            .finish()
    }
}

pub fn generate(input: &str) -> Puzzle {
    let mut map = input
        .lines()
        .map(|s| {
            let mut v = s.as_bytes().to_owned();
            v.push(EDGE);
            v.insert(0, EDGE);
            v
        })
        .collect::<Vec<_>>();
    map.insert(0, vec![EDGE; map[0].len()]);
    map.push(vec![EDGE; map[0].len()]);
    let pos = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &cell)| {
                if cell == START {
                    Some((x, y).into())
                } else {
                    None
                }
            })
        })
        .unwrap();
    let max = (map[0].len() - 1, map.len() - 1).into();
    Puzzle {
        map,
        pos,
        max,
        zero: Point { x: 1, y: 1 },
    }
}

struct Simulate {
    pos: Point,
    map: Puzzle,
    max: Point,
    zero: Point,
    direction: Vector,
    maybe_next: Point,
}

impl Simulate {
    fn new(puzzle: Puzzle) -> Self {
        let pos = puzzle.pos;
        let map = puzzle.clone();
        let max = puzzle.max;
        let zero = puzzle.zero;
        let direction = Vector { dx: 0, dy: -1 };
        let maybe_next = pos + direction;
        Self {
            pos,
            map,
            max,
            zero,
            direction,
            maybe_next,
        }
    }
}

impl Iterator for Simulate {
    // location, maybe_next_location.
    type Item = (Point, Point);

    fn next(&mut self) -> Option<Self::Item> {
        if self.map[self.maybe_next] == BLOCKED {
            // rotate 90'
            self.direction = Vector {
                dx: -self.direction.dy,
                dy: self.direction.dx,
            };
        } else {
            self.pos = self.maybe_next;
        }
        if self.pos.within(&self.max).is_none() || self.zero.within(&self.pos).is_none() {
            return None;
        }
        self.maybe_next = self.pos + self.direction;
        Some((self.pos, self.maybe_next))
    }
}

pub fn part_1(puzzle: &Puzzle) -> usize {
    let i = Simulate::new(puzzle.clone());
    let mut map = puzzle.clone();
    let mut places = 0;
    for (pos, _) in i {
        if map[pos] != VISITED {
            map[pos] = VISITED;
            places += 1;
        }
    }
    places
}

fn simulate_blockage(limit: usize, mut map: Puzzle, obstruction: Point) -> bool {
    if map[obstruction] == BLOCKED {
        return false;
    }
    map[obstruction] = BLOCKED;
    let mut loop_ = VecDeque::with_capacity(limit);

    for (steps, p_n) in Simulate::new(map).enumerate() {
        if Some(&p_n) == loop_.front() {
            return true;
        }

        loop_.push_back(p_n);
        if steps % 2 == 0 {
            //|| loop_.len() > 10000 {
            loop_.pop_front();
        }
    }
    false
}

pub fn part_2(puzzle: &Puzzle) -> usize {
    let positions = Simulate::new(puzzle.clone())
        .map(|(p, _)| p)
        .collect::<Vec<_>>();
    let limit = positions.len();
    let unique_positions = positions.into_iter().sorted().dedup().collect::<Vec<_>>();
    // dbg!(puzzle);

    let render = RenderMap {
        puzzle: puzzle.clone(),
    };

    if std::env::var("VISUALISE").is_ok() {
        App::new().run(|cx: &mut AppContext| {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(
                    Bounds::maximized(None, cx).inset(px(50.)),
                    //     Bounds::centered(
                    //     None,
                    //     size(px(max.y as f32), px(max.y as f32)),
                    //     cx,
                    // )
                )),
                ..Default::default()
            };
            cx.open_window(options, |cx| {
                cx.activate(false);
                cx.new_view(|_cx| render)
            })
            .unwrap();
        });
    }

    unique_positions
        .par_iter()
        .filter(|&&obstruction| simulate_blockage(limit, puzzle.clone(), obstruction))
        .count()
}

use gpui::*;

struct RenderMap {
    puzzle: Puzzle,
}

impl Render for RenderMap {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        // Or perhaps ...
        // RenderImage::new()
        //     .with_animation(
        //         "image_circle",
        //         Animation::new(Duration::from_secs(2))
        //             .repeat()
        //             .with_easing(bounce(ease_in_out)),
        //         |img, delta| {
        //             img.with_transformation(Transformation::rotate(percentage(delta)))
        //         },
        //     )
        //     .with_image("image_circle")
        // _cx.paint_image(bounds, corner_radii, data, frame_index, grayscale)
        // let f = font("monospace");
        let mut e = div()
            .flex()
            .flex_col()
            .bg(rgb(0x0))
            .items_center()
            .border(px(0.))
            .m(px(0.))
            .p(px(0.))
            .text_size(px(6.))
            .text_color(rgb(0xccfcf))
            .font_family("Consolas");
        for row in &self.puzzle.map {
            e = e.child(format!("{}", unsafe { str::from_utf8_unchecked(row) }));
        }
        div().flex().flex_row().size_full().justify_around().child(
            div()
                .flex()
                .flex_col()
                .size_full()
                .justify_around()
                .child(e),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{generate, part_1, part_2};

    const INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_part_1() {
        let input = generate(INPUT);
        assert_eq!(part_1(&input), 41);
    }

    #[test]
    fn test_part_2() {
        let input = generate(INPUT);
        assert_eq!(part_2(&input), 6);
    }
}
