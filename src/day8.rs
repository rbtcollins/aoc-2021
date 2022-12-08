use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day8)]
fn generate(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| c - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

// calculate visibility from one edge
fn visible<'a>(mut g: impl Iterator<Item = &'a Vec<u8>>) -> Vec<(usize, usize)> {
    let mut blocking_heights = g.next().unwrap().clone();
    let mut trees = vec![];
    for (i, row) in g.enumerate() {
        // skip start and end tree
        for (j, h) in row[1..row.len() - 1].iter().enumerate() {
            if blocking_heights[j + 1] < *h {
                blocking_heights[j + 1] = *h;
                trees.push((i + 1, j + 1))
            }
        }
    }
    trees
}

#[aoc(day8, part1)]
fn part1(input: &Vec<Vec<u8>>) -> usize {
    // calculate exterior
    let h = input.len();
    let w = input[0].len();
    let p = (h + w - 2) * 2;
    // single pass is probably possible, but constant 4x is fine.
    let transposed = {
        (0..w)
            .map(|j| (0..h).map(|i| input[i][j]).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>()
    };
    let trees1 = visible(input[..h - 1].iter());
    let trees2 = visible(input[1..].iter().rev())
        .into_iter()
        .map(|(i, j)| (h - i - 1, j))
        .collect::<Vec<_>>();
    let trees3 = visible(transposed[..w - 1].iter())
        .into_iter()
        .map(|(i, j)| (j, i))
        .collect::<Vec<_>>();
    let trees4 = visible(transposed[1..].iter().rev())
        .into_iter()
        .map(|(i, j)| (j, w - i - 1))
        .collect::<Vec<_>>();
    // eprintln!("{trees1:?}\n {trees2:?}\n {trees3:?}\n {trees4:?}");
    let interior = trees1
        .iter()
        .chain(&trees2)
        .chain(&trees3)
        .chain(&trees4)
        .sorted()
        .unique()
        .count();
    // eprintln!("{input:?} {transposed:?} {w} {h} {p} {interior}");
    p + interior
}

#[derive(Debug, Clone, Default)]
struct HeightWindow([usize; 10]);

impl HeightWindow {
    /// How far can the house see? And then updates the window given the houses height.
    fn view_distance(&mut self, house_height: &u8) -> usize {
        let house_height = *house_height as usize;
        // with a height n, and height windows [... (less than n), nth, n+1th...]
        // the house can see min([n..])
        let result = *self.0[house_height..].iter().min().unwrap();
        // and update the distance for all element
        for i in self.0.iter_mut() {
            *i += 1;
        }
        // the house height distance becomes 1
        self.0[house_height] = 1;
        result
    }
}

// calculate scores looking in one direction
fn score<'a>(g: impl Iterator<Item = &'a Vec<u8>>, scores: &mut [Vec<usize>]) {
    // walk each row multiplying score by the distance to the left
    for (i, row) in g.enumerate() {
        let mut distances = HeightWindow::default();
        for (j, h) in row.iter().enumerate() {
            scores[i][j] *= distances.view_distance(h);
        }
    }
}

#[aoc(day8, part2)]
fn part2(input: &[Vec<u8>]) -> usize {
    let input = &mut input.to_vec();
    let h = input.len();
    let w = input[0].len();
    // single pass is probably possible, but constant 4x is fine.
    let mut scores = vec![vec![1usize; w]; h];
    score(input.iter(), &mut scores);

    // reverse the content of the rows
    scores.iter_mut().for_each(|r| r.reverse());
    // scores.reverse();
    score(
        input.iter_mut().map(|r| {
            r.reverse();
            &*r
        }),
        &mut scores,
    );

    let mut scores = {
        (0..w)
            .map(|j| (0..h).map(|i| scores[i][j]).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>()
    };
    let input = &mut {
        (0..w)
            .map(|j| (0..h).map(|i| input[i][j]).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>()
    };
    score(input.iter(), &mut scores);

    // reverse the content of the rows
    scores.iter_mut().for_each(|r| r.reverse());
    // scores.reverse();
    score(
        input.iter_mut().map(|r| {
            r.reverse();
            &*r
        }),
        &mut scores,
    );

    // max from interior
    *scores[1..scores.len() - 1]
        .iter()
        .flat_map(|r| r[1..r.len() - 1].iter())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn part1() {
        let input = super::generate(SAMPLE);
        assert_eq!(21, super::part1(&input));
    }

    #[test]
    fn part2() {
        let input = super::generate(SAMPLE);
        assert_eq!(8, super::part2(&input));
    }
}
