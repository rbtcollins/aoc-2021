use lending_iterator::prelude::*;

pub fn generate(input: &str) -> Vec<Vec<Vec<usize>>> {
    input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|p| {
                    p.split(',')
                        .map(|n| str::parse(n.trim()).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part_1(input: &[Vec<Vec<usize>>]) -> usize {
    // find bounds

    let t = input.iter().flatten().map(|pos| pos[0]);
    let min = t.clone().min().unwrap() - 1; // start at 1
    let max = t.max().unwrap() - min;
    let sand_start = 500 - min;
    // [y][x]
    let max_row = input.iter().flatten().map(|pos| pos[1]).max().unwrap() + 2;
    let mut arena = vec![vec![false; max + 1]; max_row];
    for line in input.iter() {
        for pairs in line.windows(2) {
            let start = &pairs[0];
            let end = &pairs[1];
            // draw horizontally; if it is a vertical line, these will draw one point.
            for x in start[0]..=end[0] {
                arena[start[1]][x - min] = true;
            }
            for x in end[0]..=start[0] {
                arena[start[1]][x - min] = true;
            }
            // draw vertically
            for row in arena.iter_mut().take(end[1] + 1).skip(start[1]) {
                row[start[0] - min] = true;
            }
            for row in arena.iter_mut().take(start[1] + 1).skip(end[1]) {
                row[start[0] - min] = true;
            }

            // for y in end[1]..=start[1] {
            //     arena[y][start[0] - min] = true;
            // }
        }
    }
    // simulate sand
    let mut grains = 0;
    'done: loop {
        // eprintln!("Grain {grains}");
        // for row in arena.iter() {
        //     eprintln!(
        //         "{:?}",
        //         row.iter()
        //             .map(|t| if *t { '#' } else { '.' })
        //              .collect::<String>()
        //     );
        // }
        let mut x = sand_start;
        let mut rows_iter = arena.windows_mut::<2>();
        let mut row = 1;
        while let Some(&mut [ref mut curr, ref next]) = rows_iter.next() {
            // eprintln!("{curr:?}");
            // eprintln!("{next:?}");
            row += 1;
            if row == max_row {
                break 'done;
            }
            if !next[x] {
                continue;
            }
            if !next[x - 1] {
                x -= 1;
                continue;
            }
            if !next[x + 1] {
                x += 1;
                continue;
            }
            grains += 1;
            curr[x] = true;
            break;
        }
    }
    grains
}

pub fn part_2(input: &[Vec<Vec<usize>>]) -> usize {
    // find bounds
    let max_row = input.iter().flatten().map(|pos| pos[1]).max().unwrap() + 3;
    let t = input.iter().flatten().map(|pos| pos[0]);
    // start offset by number of rows to allow the pyramid to build
    let min = t.clone().min().unwrap() - 1 - max_row;
    let width = t.max().unwrap() - min + max_row;
    let sand_start = 500 - min;
    // [y][x]
    let mut arena = vec![vec![false; width + 1]; max_row];
    for line in input.iter() {
        for pairs in line.windows(2) {
            let start = &pairs[0];
            let end = &pairs[1];
            // draw horizontally; if it is a vertical line, these will draw one point.
            for x in start[0]..=end[0] {
                arena[start[1]][x - min] = true;
            }
            for x in end[0]..=start[0] {
                arena[start[1]][x - min] = true;
            }
            // draw vertically
            for row in arena.iter_mut().take(end[1] + 1).skip(start[1]) {
                row[start[0] - min] = true;
            }
            for row in arena.iter_mut().take(start[1] + 1).skip(end[1]) {
                row[start[0] - min] = true;
            }

            // for y in end[1]..=start[1] {
            //     arena[y][start[0] - min] = true;
            // }
        }
    }
    arena[max_row - 1].iter_mut().for_each(|c| *c = true);
    // simulate sand
    let mut grains = 0;
    'done: loop {
        // eprintln!("Grain {grains}");
        // for row in arena.iter() {
        //     eprintln!(
        //         "{:?}",
        //         row.iter()
        //             .map(|t| if *t { '#' } else { '.' })
        //             .collect::<String>()
        //     );
        // }
        let mut x = sand_start;
        let mut rows_iter = arena.windows_mut::<2>();
        while let Some(&mut [ref mut curr, ref next]) = rows_iter.next() {
            // eprintln!("{curr:?}");
            // eprintln!("{next:?}");
            if curr[x] {
                break 'done;
            }
            if !next[x] {
                continue;
            }
            if !next[x - 1] {
                x -= 1;
                continue;
            }
            if !next[x + 1] {
                x += 1;
                continue;
            }
            grains += 1;
            curr[x] = true;
            break;
        }
    }
    grains
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

    #[test]
    fn part_1() {
        let input = super::generate(SAMPLE);
        assert_eq!(24, super::part_1(&input));
    }

    #[test]
    fn part_2() {
        let input = super::generate(SAMPLE);
        assert_eq!(93, super::part_2(&input));
    }
}
