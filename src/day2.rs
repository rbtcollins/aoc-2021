use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, PartialEq, enum_utils::FromStr)]
enum Elf {
    A,
    B,
    C,
}

#[derive(Debug, PartialEq, enum_utils::FromStr)]
enum Advice {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum RPS {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl From<&Advice> for RPS {
    fn from(value: &Advice) -> Self {
        match value {
            Advice::X => RPS::Rock,
            Advice::Y => RPS::Paper,
            Advice::Z => RPS::Scissors,
        }
    }
}

impl From<&Elf> for RPS {
    fn from(value: &Elf) -> Self {
        match value {
            Elf::A => RPS::Rock,
            Elf::B => RPS::Paper,
            Elf::C => RPS::Scissors,
        }
    }
}

impl TryFrom<&i32> for RPS {
    type Error = String;

    fn try_from(value: &i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => RPS::Rock,
            1 => RPS::Paper,
            2 => RPS::Scissors,
            _ => return Err("Bad value".into()),
        })
    }
}

impl RPS {
    fn choose_shape(elf: &Elf, advice: &Advice) -> Self {
        // Advice::X - lose, then draw and win
        let offset = RPS::from(advice) as i32 - 1;
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
fn outcome(elf: &Elf, mine: &RPS) -> usize {
    3 * ((3 + 1 + mine.clone() as usize - RPS::from(elf) as usize) % 3)
}

#[aoc_generator(day2)]
fn generate(input: &str) -> Vec<(Elf, Advice)> {
    input
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(Elf, Advice)]) -> usize {
    input
        .iter()
        .map(|(elf, advice)| {
            let mine = RPS::from(advice);
            mine.shape_score() + outcome(elf, &mine)
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[(Elf, Advice)]) -> usize {
    input
        .iter()
        .map(|(elf, advice)| {
            let mine = &RPS::choose_shape(elf, advice);
            mine.shape_score() + outcome(elf, mine)
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
        assert_eq!(6, outcome(&Elf::A, &RPS::from(&Advice::Y)));
        assert_eq!(2, RPS::from(&Advice::Y).shape_score());
        assert_eq!(0, outcome(&Elf::B, &RPS::from(&Advice::X)));
        assert_eq!(3, outcome(&Elf::C, &RPS::from(&Advice::Z)));

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
        assert_eq!(RPS::Rock, RPS::choose_shape(&Elf::A, &Advice::Y));
    }
}
