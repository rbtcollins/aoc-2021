use aoc_runner_derive::{aoc, aoc_generator};
use serde::Serialize;
use serde_plain::derive_display_from_serialize;

#[derive(Clone, Debug, PartialEq, enum_utils::FromStr, Serialize)]
enum Elf {
    A,
    B,
    C,
}
derive_display_from_serialize!(Elf);

#[derive(Debug, PartialEq, enum_utils::FromStr, Serialize)]
enum Advice {
    X,
    Y,
    Z,
}
derive_display_from_serialize!(Advice);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum RockPaperScissors {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl From<&Advice> for RockPaperScissors {
    fn from(value: &Advice) -> Self {
        match value {
            Advice::X => RockPaperScissors::Rock,
            Advice::Y => RockPaperScissors::Paper,
            Advice::Z => RockPaperScissors::Scissors,
        }
    }
}

impl From<&Elf> for RockPaperScissors {
    fn from(value: &Elf) -> Self {
        match value {
            Elf::A => RockPaperScissors::Rock,
            Elf::B => RockPaperScissors::Paper,
            Elf::C => RockPaperScissors::Scissors,
        }
    }
}

impl TryFrom<&i32> for RockPaperScissors {
    type Error = String;

    fn try_from(value: &i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => RockPaperScissors::Rock,
            1 => RockPaperScissors::Paper,
            2 => RockPaperScissors::Scissors,
            _ => return Err("Bad value".into()),
        })
    }
}

#[cfg(test)]
impl RockPaperScissors {
    fn choose_shape(elf: &Elf, advice: &Advice) -> Self {
        // Advice::X - lose, then draw and win
        let offset = RockPaperScissors::from(advice) as i32 - 1;
        let result = (elf.clone() as i32 + offset + 3) % 3;
        Self::try_from(&result).unwrap()
    }

    fn shape_score(&self) -> usize {
        self.clone() as usize + 1
    }
}

// want win == 6
// want draw == 3
// want lose == 0
//
// 0=R, 1=P, 2=S, R>P, P>S, S>R, P<R, S<P, R<S
//
// 3*((A - E +1+3)%3) ?
// E vs A
// should be 0(0):
// R v S = 0 vs 2 = 2-0+1 =0
// P v R = 1 vs 0 = 0-1+1 =0
// S v P = 2 vs 1 = 1-2+1 =0
// should be 1(3)
// N v N =          N-N +1 +3%3 =1  XXX
// should be 2(6)
// R v P = 0 vs 1 = 1-0 + 1 = 2
// P v S = 1 vs 2 = 2-1 + 1 = 2
// S v R = 2 vs 0 = 0-1 + 1 = 2
#[cfg(test)]
fn outcome(elf: &Elf, mine: &RockPaperScissors) -> usize {
    3 * ((3 + 1 + mine.clone() as usize - RockPaperScissors::from(elf) as usize) % 3)
}

// #[aoc_generator(day2)]
// fn generate(input: &str) -> Vec<(Elf, Advice)> {
//     input
//         .lines()
//         .flat_map(|line| line.split_once(' '))
//         .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
//         .collect()
// }

#[aoc_generator(day2)]
fn generate(input: &str) -> String {
    let mut owned = input.to_owned();
    if !input.ends_with('\n') {
        owned.push('\n');
    }
    owned
}

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let mut lookup = vec![0; 36];

    lookup[0] = 4;
    lookup[16] = 8;
    lookup[32] = 3;
    lookup[1] = 1;
    lookup[17] = 5;
    lookup[33] = 9;
    lookup[2] = 7;
    lookup[18] = 2;
    lookup[34] = 6;
    input
        .as_bytes()
        .chunks_exact(4)
        .map(|chunk| u32::from_ne_bytes(chunk.try_into().unwrap()))
        .map(|v| {
            // low order
            let elf = (v & 0x000000ff) as usize - b'A' as usize;
            // high order
            let advice = ((v & 0x00ff0000) >> 12) as usize - ((b'X' as usize) << 4);
            unsafe { lookup.get_unchecked(elf | advice) }
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    let mut lookup = vec![0; 36];

    lookup[0] = 3;
    lookup[16] = 4;
    lookup[32] = 8;
    lookup[1] = 1;
    lookup[17] = 5;
    lookup[33] = 9;
    lookup[2] = 2;
    lookup[18] = 6;
    lookup[34] = 7;
    input
        .as_bytes()
        .chunks_exact(4)
        .map(|chunk| u32::from_ne_bytes(chunk.try_into().unwrap()))
        .map(|v| {
            // low order
            let elf = (v & 0x000000ff) as usize - b'A' as usize;
            // high order
            let advice = ((v & 0x00ff0000) >> 12) as usize - ((b'X' as usize) << 4);
            unsafe { lookup.get_unchecked(elf | advice) }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(
            15,
            part1(&generate(
                "A Y
B X
C Z
"
            ))
        );
    }

    #[test]
    fn scoring() {
        assert_eq!(6, outcome(&Elf::A, &RockPaperScissors::from(&Advice::Y)));
        assert_eq!(2, RockPaperScissors::from(&Advice::Y).shape_score());
        assert_eq!(0, outcome(&Elf::B, &RockPaperScissors::from(&Advice::X)));
        assert_eq!(3, outcome(&Elf::C, &RockPaperScissors::from(&Advice::Z)));

        // In the first round, your opponent will choose Rock (A), and you should choose Paper (Y).
        // This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
        // In the second round, your opponent will choose Paper (B), and you should choose Rock (X).
        // This ends in a loss for you with a score of 1 (1 + 0).
        // The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
    }

    #[test]
    fn test_part2() {
        //"Anyway, the second column says how the round needs to end: X means you
        //need to lose, Y means you need to end the round in a draw, and Z means you
        //need to win. Good luck!" The total score is still calculated in the same
        //way, but now you need to figure out what shape to choose so the round ends
        //as indicated. The example above now goes like this: In the first round,
        //your opponent will choose Rock (A), and you need the round to end in a
        //draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4. In
        //the second round, your opponent will choose Paper (B), and you choose Rock
        //so you lose (X) with a score of 1 + 0 =
        //1. In the third round, you will defeat your opponent's Scissors with Rock
        //for a score of 1 + 6 = 7. Now that you're correctly decrypting the ultra
        //top secret strategy guide, you would get a total score of 12.
        assert_eq!(
            12,
            part2(&generate(
                "A Y
B X
C Z
"
            ))
        );
    }

    #[test]
    fn test_choose_shape() {
        assert_eq!(
            RockPaperScissors::Rock,
            RockPaperScissors::choose_shape(&Elf::A, &Advice::Y)
        );
    }

    // #[test]
    // fn experiment() {
    //     for v in "A X\n"
    //         .as_bytes()
    //         .chunks_exact(4)
    //         .map(|chunk| u32::from_ne_bytes(chunk.try_into().unwrap()))
    //     {
    //         eprintln!(
    //             "{} {} {} {} {} {}",
    //             v,
    //             b'A',
    //             ((v & 0xff000000) >> 24) as u8, // - 'A' as u8,
    //             ((v & 0x00ff0000) >> 16) as u8 - b'X',
    //             ((v & 0x0000ff00) >> 8) as u8, // - 'X' as u8,
    //             (v & 0x000000ff) as u8 - b'A'
    //         );
    //     }
    // }

    #[test]
    fn gen_part1() {
        // match eprintln!(
        //         "{} {} {} {} {} {}",
        //         v,
        //         'A' as u8,
        //         ((v & 0xff000000) >> 24) as u8, // - 'A' as u8,
        //         ((v & 0x00ff0000) >> 16) as u8 - 'X' as u8,
        //         ((v & 0x0000ff00) >> 8) as u8, // - 'X' as u8,
        //         ((v & 0x000000ff) >> 0) as u8 - 'A' as u8
        //     );
        // Vec::default().push
        eprintln!(
            "let mut lookup=Vec::with_capacity(36);
lookup.resize(36, 0);
"
        );
        // // low order
        // let elf = ((v & 0x000000ff) >> 0) as u8 - 'A' as u8;
        // // high order
        // let advice = ((v & 0x00ff0000) >> 12) as u8 - (('X' as u8) << 4);
        // match elf|advice {{
        // "
        //         );
        for elf in [Elf::A, Elf::B, Elf::C] {
            for advice in [Advice::X, Advice::Y, Advice::Z] {
                let mine = RockPaperScissors::from(&advice);
                eprintln!(
                    "lookup[{}] = {};",
                    (elf.clone() as u8) | ((advice as u8) << 4),
                    mine.shape_score() + outcome(&elf, &mine)
                );
            }
        }
        // eprintln!("v => unreachable!(\"{{}}\", v)");
        // eprintln!("}}");
    }

    #[test]
    fn gen_part2() {
        for elf in [Elf::A, Elf::B, Elf::C] {
            for advice in [Advice::X, Advice::Y, Advice::Z] {
                let mine = &RockPaperScissors::choose_shape(&elf, &advice);
                eprintln!(
                    "lookup[{}] = {};",
                    (elf.clone() as u8) | ((advice as u8) << 4),
                    mine.shape_score() + outcome(&elf, mine)
                );
            }
        }
        // eprint!("_ => unreachable!(),");
    }
}
