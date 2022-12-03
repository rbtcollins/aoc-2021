use aoc_runner_derive::{aoc, aoc_generator};

const fn char_to_bit(char: u8) -> u64 {
    0x1 << (if char > b'Z' {
        char - b'a' + 1 // a -> 1
    } else {
        char - b'A' + 27 // A -> 27
    })
}

const fn gen_lookup() -> [u64; 256] {
    let mut lookup = [0u64; 256];
    let mut i = b'a';
    while i <= b'z' {
        lookup[i as usize] = char_to_bit(i);
        i += 1;
    }
    i = b'A';
    while i <= b'Z' {
        lookup[i as usize] = char_to_bit(i);
        i += 1;
    }
    lookup
}

const LOOKUP: [u64; 256] = gen_lookup();

#[aoc_generator(day3)]
fn generate(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| LOOKUP[*b as usize]).collect())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u64>]) -> u32 {
    input
        .iter()
        .map(|rs| {
            let (l, r) = rs.split_at(rs.len() >> 1);

            let common = l.iter().fold(0u64, |val, item| val | item)
                & r.iter().fold(0u64, |val, item| val | item);
            common.trailing_zeros()
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u64>]) -> u32 {
    input
        .chunks_exact(3)
        .map(|c| {
            let common = c[0].iter().fold(0u64, |val, item| val | item)
                & c[1].iter().fold(0u64, |val, item| val | item)
                & c[2].iter().fold(0u64, |val, item| val | item);
            common.trailing_zeros()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&generate(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
            )),
            157
        );
    }

    #[test]
    fn test_part1_individual() {
        assert_eq!(part1(&generate("vJrwpWtwJgWrhcsFMMfFFhFp",)), 16);
        assert_eq!(part1(&generate("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",)), 38);
        assert_eq!(part1(&generate("PmmdzqPrVvPwwTWBwg",)), 42);
        assert_eq!(part1(&generate("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",)), 22);
        assert_eq!(part1(&generate("ttgJtRGJQctTZtZT",)), 20);
        assert_eq!(part1(&generate("CrZsJsPPZsGzwwsLwLmpwMDw",)), 19);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&generate(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
            )),
            70
        );
    }
}
