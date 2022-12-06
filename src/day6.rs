use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn generate(input: &str) -> Vec<char> {
    input.chars().collect::<Vec<_>>()
}

#[aoc(day6, part1)]
fn part1(input: &Vec<char>) -> u32 {
    find_marker(input, 4)
}

#[aoc(day6, part2)]
fn part2(input: &Vec<char>) -> u32 {
    find_marker(input, 14)
}

fn find_marker(input: &Vec<char>, length: usize) -> u32 {
    let mut last_dup = 0;
    for p in 1..input.len() {
        if last_dup >= p {
            continue;
        }

        // eprintln!("{last_dup}..{p}, {:?}", &input[last_dup..=p]);
        for l in (last_dup..p).rev() {
            if input[l] == input[p] {
                // duplicate
                // eprintln!("Dup at {l} {p} {:?}", &input[l..=p]);
                last_dup = l + 1;
                break;
            }
            if p - last_dup == length {
                return p as u32;
            }
            // eprintln!("{l},{p}");
        }
    }
    0
}

#[cfg(test)]
mod tests {

    const SAMPLES: &[&str] = &[
        r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#,
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
    const P1_R: &[u32] = &[7, 5, 6, 10, 11];
    const P2_R: &[u32] = &[19, 23, 23, 29, 26];

    #[test]
    fn part1() {
        for (sample, result) in SAMPLES.iter().zip(P1_R) {
            let input = super::generate(sample);
            assert_eq!(result, &super::part1(&input));
        }
    }

    #[test]
    fn part2() {
        for (sample, result) in SAMPLES.iter().zip(P2_R) {
            let input = super::generate(sample);
            assert_eq!(result, &super::part2(&input));
        }
    }
}
