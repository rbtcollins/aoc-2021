use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn generate(input: &str) -> [u64; 9] {
    // index=(days+pos %7) , value=number of fish.
    let mut result = [0; 9];
    for fish in input.split(',').flat_map(|s| s.parse::<usize>()) {
        result[fish] += 1;
    }
    result
}

fn run_sim(input: &[u64; 9], steps: usize) -> u64 {
    // load into register-capable vars
    let [mut t0, mut t1, mut t2, mut t3, mut t4, mut t5, mut t6, mut t7, mut t8] = input;
    let mut tmp: u64;
    for _ in 0..steps {
        // advance time
        tmp = t0;
        t0 = t1;
        t1 = t2;
        t2 = t3;
        t3 = t4;
        t4 = t5;
        t5 = t6;
        t6 = tmp + t7;
        t7 = t8;
        t8 = tmp;
    }
    // return
    t0 + t1 + t2 + t3 + t4 + t5 + t6 + t7 + t8
}

#[aoc(day6, part1)]
fn part1(input: &[u64; 9]) -> u64 {
    run_sim(input, 80)
}

#[aoc(day6, part2)]
fn part2(input: &[u64; 9]) -> u64 {
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
