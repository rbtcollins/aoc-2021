use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default, Clone, Debug)]
struct Input {
    /// index=(days+pos %7) , value=number of fish.
    fish: [usize; 9],
    zero: usize,
}

impl Input {
    #[inline]
    fn step(&mut self) {
        self.zero = (self.zero + 1) % 7;
        // move the two slow cases: 7's become 6, 8's become 7's.
        let pos_6 = (self.zero + 6) % 7;
        let new_fish = self.fish[pos_6];
        self.fish[pos_6] += self.fish[7];
        self.fish[7] = self.fish[8];
        // breeding - original 6's -> 8's.
        self.fish[8] = new_fish;
    }
}

#[aoc_generator(day6)]
fn generate(input: &str) -> Input {
    let mut result = Input {
        fish: [0; 9],
        zero: 0,
    };
    for fish in input.split(',').flat_map(|s| s.parse::<usize>()) {
        result.fish[fish] += 1;
    }
    result
}

fn run_sim(input: &Input, steps: usize) -> usize {
    let mut state = input.clone();
    for _ in 0..steps {
        state.step();
    }
    state.fish.iter().sum()
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> usize {
    run_sim(input, 80)
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> usize {
    run_sim(input, 256)
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"3,4,3,1,2"#;

    #[test]
    fn part1() {
        let input = super::generate(SAMPLE);
        assert_eq!(5934, super::part1(&input));
    }

    #[test]
    fn part2() {
        let input = super::generate(SAMPLE);
        assert_eq!(26984457539, super::part2(&input));
    }
}
