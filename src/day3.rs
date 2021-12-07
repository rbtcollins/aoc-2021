use std::rc::Rc;
use std::str;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn generate(input: &str) -> Vec<Rc<Vec<u8>>> {
    input
        .lines()
        .map(|s| Rc::new(s.as_bytes().to_owned()))
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Rc<Vec<u8>>]) -> usize {
    let mut totals = vec![0; input[0].len()];
    for diagnostic in input {
        for pos in 0..diagnostic.len() {
            let val: i8 = (diagnostic[pos] - b'0').try_into().unwrap();
            totals[pos] = totals[pos] + val + val - 1;
        }
    }

    // totals[0] < 0 if most common bit is 0. >0 if MCB is 1.
    let mut epsilon = 0;
    let mut gamma = 0;
    for val in totals {
        epsilon <<= 1;
        gamma <<= 1;
        match val {
            i if i < 0 => epsilon += 1,
            i if i > 0 => gamma += 1,
            _ => panic!("aiiee"),
        }
    }
    gamma * epsilon
}

fn split_on_bit(input: &[Rc<Vec<u8>>], pos: usize) -> (Vec<Rc<Vec<u8>>>, Vec<Rc<Vec<u8>>>) {
    let mut zeros = vec![];
    let mut ones = vec![];
    for diagnostic in input {
        match diagnostic[pos] {
            b'0' => zeros.push(diagnostic.clone()),
            _ => ones.push(diagnostic.clone()),
        }
    }
    (zeros, ones)
}

#[aoc(day3, part2)]
fn part2(input: &[Rc<Vec<u8>>]) -> usize {
    // oxy: most common, tie take 1
    // co2: least common. tie take 0
    let (mut oxy, mut co2) = {
        let (zeros, ones) = split_on_bit(input, 0);
        match zeros.len() as isize - ones.len() as isize {
            i if i > 0 => (zeros, ones),
            i if i < 0 => (ones, zeros),
            _ => (ones, zeros),
        }
    };
    let mut pos = 1;
    loop {
        if oxy.len() == 1 {
            break;
        }
        let mut new_oxy = {
            let (zeros, ones) = split_on_bit(&oxy, pos);
            match zeros.len() as isize - ones.len() as isize {
                i if i > 0 => (zeros),
                i if i < 0 => (ones),
                _ => (ones),
            }
        };
        std::mem::swap(&mut new_oxy, &mut oxy);
        pos += 1;
    }
    // we could factor these three similar things but meh.
    let mut pos = 1;
    loop {
        if co2.len() == 1 {
            break;
        }
        let (zeros, ones) = split_on_bit(&co2, pos);
        co2 = match zeros.len() as isize - ones.len() as isize {
            i if i > 0 => (ones),
            i if i < 0 => (zeros),
            _ => (zeros),
        };
        pos += 1;
    }
    usize::from_str_radix(str::from_utf8(&oxy[0]).unwrap(), 2).unwrap()
        * usize::from_str_radix(str::from_utf8(&co2[0]).unwrap(), 2).unwrap()
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

    #[test]
    fn part1() {
        let input = super::generate(SAMPLE);
        assert_eq!(198, super::part1(&input));
    }

    #[test]
    fn part2() {
        let input = super::generate(SAMPLE);
        assert_eq!(230, super::part2(&input));
    }
}
