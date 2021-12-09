use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Default, Clone, Debug)]
struct Display {
    samples: Vec<String>,
    digits: Vec<String>,
}

#[derive(Default, Clone, Debug)]
struct Input {
    displays: Vec<Display>,
}

impl Input {}

#[aoc_generator(day8)]
fn generate(input: &str) -> Input {
    Input {
        displays: input
            .lines()
            .map(|l| {
                let s = l.split('|').collect::<Vec<_>>();
                let samples: Vec<String> = s[0]
                    .trim()
                    .split(' ')
                    .map(String::from)
                    .map(sort_string)
                    .collect();
                let digits: Vec<String> = s[1]
                    .trim()
                    .split(' ')
                    .map(String::from)
                    .map(sort_string)
                    .collect();
                Display { samples, digits }
            })
            .collect(),
    }
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    input
        .displays
        .iter()
        .flat_map(|d| {
            d.digits.iter().flat_map(|d| {
                if d.len() == 2 {
                    Some(1)
                } else if d.len() == 4 {
                    Some(4)
                } else if d.len() == 3 {
                    Some(3)
                } else if d.len() == 7 {
                    Some(8)
                } else {
                    None
                }
            })
        })
        .count()
}

fn index(digit: char) -> usize {
    (digit as u32 - 'a' as u32) as usize
}

fn to_digits(chars: &str, mapping: &[char]) -> String {
    let mut s = String::new();
    for ch in chars.chars() {
        s.push(mapping[index(ch)]);
    }

    sort_string(s)
}

fn sort_string(string: String) -> String {
    string.chars().sorted().collect()
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> usize {
    input
        .displays
        .iter()
        .map(|d| {
            let mut solutions = vec![String::new(); 10];
            // trivial cases needed to solve more complicated ones
            for s in d.samples.iter() {
                if s.len() == 2 {
                    solutions[1] = s.clone();
                } else if s.len() == 3 {
                    solutions[7] = s.clone();
                } else if s.len() == 4 {
                    solutions[4] = s.clone();
                } else if s.len() == 7 {
                    solutions[8] = s.clone();
                }
            }
            let mut histogram = vec![0; 7];
            // segments map to a-g
            // 0/ /2/3/ /5/6/7/8/9 share a 8 2
            // 0/ / / /4/5/6/ /8/9 share b 6 1
            // 0/1/2/3/4/ / /7/8/9 share c 8 2
            //  / /2/3/4/5/6/ /8/9 share d 7 2
            // 0/ /2/ / / /6/ /8/  share e 4 1
            // 0/1/ /3/4/5/6/7/8/9 share f 9 1
            // 0/ /2/3/ /5/6/ /8/9 share g 7 2
            // -----------
            //
            // present in 9 = f position
            // present in 8 -> a or c
            // present in 7 -> d or g
            // present in 6 -> b position
            // present in 4 -> e position
            // present in 8 and in '1' -> c
            // present in 8 and not in '1' -> a
            // present in 7 and in '0' -> g
            // present in 7 and no
            for sample in d.samples.iter() {
                // populate histogram
                for char in sample.chars() {
                    histogram[index(char)] += 1;
                }
            }

            let mut mapping = vec!['z'; 7];
            for (i, v) in histogram.iter().enumerate() {
                let c = unsafe { char::from_u32_unchecked('a' as u32 + i as u32) };
                let v = *v;
                if v == 9 {
                    mapping[index('f')] = c;
                } else if v == 6 {
                    mapping[index('b')] = c;
                } else if v == 4 {
                    mapping[index('e')] = c;
                } else if v == 8 {
                    // a or c
                    if solutions[1].contains(c) {
                        mapping[index('c')] = c;
                    } else {
                        mapping[index('a')] = c;
                    }
                } else if v == 7 {
                    // d or g
                    if solutions[4].contains(c) {
                        mapping[index('d')] = c;
                    } else {
                        mapping[index('g')] = c;
                    }
                }
            }
            let mut resolved = vec![];

            resolved.push(to_digits("abcefg", &mapping)); // 0
            resolved.push(to_digits("cf", &mapping)); // 1
            resolved.push(to_digits("acdeg", &mapping)); // 2
            resolved.push(to_digits("acdfg", &mapping)); // 3
            resolved.push(to_digits("bcdf", &mapping)); // 4
            resolved.push(to_digits("abdfg", &mapping)); // 5
            resolved.push(to_digits("abdefg", &mapping)); // 6
            resolved.push(to_digits("acf", &mapping)); // 7
            resolved.push(to_digits("abcdefg", &mapping)); // 8
            resolved.push(to_digits("abcdfg", &mapping)); // 9
            let result = d
                .digits
                .iter()
                .map(|s| resolved.iter().position(|s2| s2 == s))
                .map(|o| {
                    assert!(o.is_some());
                    o.unwrap()
                })
                .collect::<Vec<_>>();
            result.iter().fold(0, |a, i| a * 10 + *i)
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

    #[test]
    fn part1() {
        let input = super::generate(SAMPLE);
        assert_eq!(26, super::part1(&input));
    }

    #[test]
    fn part2() {
        let input = super::generate(SAMPLE);
        assert_eq!(61229, super::part2(&input));
    }

    #[test]
    fn index() {
        assert_eq!(0, super::index('a'));
        assert_eq!(6, super::index('g'));
    }
}
