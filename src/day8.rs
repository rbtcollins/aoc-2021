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

// #[aoc(day8, part2)]
// fn part2(input: &Input) -> usize {
//     input
//         .displays
//         .iter()
//         .map(|d| {
//             let mut solutions = vec![String::new(); 10];
//             // trivial cases needed to solve more complicated ones
//             for s in d.samples.iter() {
//                 if s.len() == 2 {
//                     solutions[1] = s.clone();
//                 } else if s.len() == 3 {
//                     solutions[7] = s.clone();
//                 } else if s.len() == 4 {
//                     solutions[4] = s.clone();
//                 } else if s.len() == 7 {
//                     solutions[8] = s.clone();
//                 }
//             }
//             let mut histogram = vec![0; 7];
//             // segments map to a-g
//             // 0/ /2/3/ /5/6/7/8/9 share a 8 2
//             // 0/ / / /4/5/6/ /8/9 share b 6 1
//             // 0/1/2/3/4/ / /7/8/9 share c 8 2
//             //  / /2/3/4/5/6/ /8/9 share d 7 2
//             // 0/ /2/ / / /6/ /8/  share e 4 1
//             // 0/1/ /3/4/5/6/7/8/9 share f 9 1
//             // 0/ /2/3/ /5/6/ /8/9 share g 7 2
//             // -----------
//             //
//             // present in 9 = f position
//             // present in 8 -> a or c
//             // present in 7 -> d or g
//             // present in 6 -> b position
//             // present in 4 -> e position
//             // present in 8 and in '1' -> c
//             // present in 8 and not in '1' -> a
//             // present in 7 and in '0' -> g
//             // present in 7 and no
//             for sample in d.samples.iter() {
//                 // populate histogram
//                 for char in sample.chars() {
//                     histogram[index(char)] += 1;
//                 }
//             }

//             let mut mapping = vec!['z'; 7];
//             for (i, v) in histogram.iter().enumerate() {
//                 let c = unsafe { char::from_u32_unchecked('a' as u32 + i as u32) };
//                 let v = *v;
//                 if v == 9 {
//                     mapping[index('f')] = c;
//                 } else if v == 6 {
//                     mapping[index('b')] = c;
//                 } else if v == 4 {
//                     mapping[index('e')] = c;
//                 } else if v == 8 {
//                     // a or c
//                     if solutions[1].contains(c) {
//                         mapping[index('c')] = c;
//                     } else {
//                         mapping[index('a')] = c;
//                     }
//                 } else if v == 7 {
//                     // d or g
//                     if solutions[4].contains(c) {
//                         mapping[index('d')] = c;
//                     } else {
//                         mapping[index('g')] = c;
//                     }
//                 }
//             }
//             let mut resolved = vec![];

//             resolved.push(to_digits("abcefg", &mapping)); // 0
//             resolved.push(to_digits("cf", &mapping)); // 1
//             resolved.push(to_digits("acdeg", &mapping)); // 2
//             resolved.push(to_digits("acdfg", &mapping)); // 3
//             resolved.push(to_digits("bcdf", &mapping)); // 4
//             resolved.push(to_digits("abdfg", &mapping)); // 5
//             resolved.push(to_digits("abdefg", &mapping)); // 6
//             resolved.push(to_digits("acf", &mapping)); // 7
//             resolved.push(to_digits("abcdefg", &mapping)); // 8
//             resolved.push(to_digits("abcdfg", &mapping)); // 9
//             let result = d
//                 .digits
//                 .iter()
//                 .map(|s| resolved.iter().position(|s2| s2 == s))
//                 .map(|o| {
//                     assert!(o.is_some());
//                     o.unwrap()
//                 })
//                 .collect::<Vec<_>>();
//             result.iter().fold(0, |a, i| a * 10 + *i)
//         })
//         .sum::<usize>()
// }

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

    // #[test]
    // fn part2() {
    //     let input = super::generate(SAMPLE);
    //     assert_eq!(61229, super::part2(&input));
    // }

    // #[test]
    // fn index() {
    //     assert_eq!(0, super::index('a'));
    //     assert_eq!(6, super::index('g'));
    // }
}
