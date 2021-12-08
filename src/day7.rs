use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Default, Clone, Debug)]
struct Input {
    positions: Vec<isize>,
}

impl Input {}

#[aoc_generator(day7)]
fn generate(input: &str) -> Input {
    let mut result = Input {
        positions: input.split(',').flat_map(|s| s.parse::<isize>()).collect(),
    };
    result.positions.sort_unstable();
    result
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> isize {
    let positions = &input.positions;
    let mut val = 0;
    let mut current_cost: isize = positions.iter().sum();
    let mut right: isize = -(positions.len() as isize);
    for (left, new_val) in positions.iter().enumerate() {
        let position_increment = new_val - val;
        // cost of everything to the left increases by position_increment.
        // cost of everything to the right decreases by position_increment
        let new_cost = current_cost + position_increment * (left as isize + right);
        // we remove 1 items from right
        right += 1;
        // capture the cost
        if new_cost > current_cost {
            return current_cost;
        }
        current_cost = new_cost;
        val = *new_val;
    }
    current_cost
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> isize {
    let positions = &input.positions;
    let mut right_i = positions.len() as isize;
    // pass 0: sum distances
    let mut right_sid: isize = positions.iter().sum();
    // pass 1: transform to pos:count
    let mut groups: Vec<isize> = vec![0; *positions.last().unwrap() as usize + 1];
    for (k, c) in input
        .positions
        .iter()
        .group_by(|k| *k)
        .into_iter()
        .map(|(k, g)| (*k, g.into_iter().count() as isize))
    {
        groups[k as usize] = c;
    }
    // pass 2: accumulate the cost from 0
    let mut right_v = groups
        .iter()
        .enumerate()
        .map(|(k, c)| k as isize * (k as isize + 1) / 2 * c)
        .sum();
    // pass 3: rolling calculation to find best point
    let mut pos = 0;
    let mut left_i = 0;
    let mut left_sid = 0;
    let mut left_v = 0;
    let mut current_i = 0;
    let mut current_cost = right_v;
    for (new_pos, count) in groups.iter().enumerate() {
        let new_pos = new_pos as isize;
        let step = new_pos - pos;
        // apply previous indices as they move out from 0.
        left_i += current_i;
        left_v += step * (2 * left_sid + (step + 1) * left_i) / 2;
        left_sid += step * left_i;
        // save new indices
        current_i = *count;
        right_v += step * ((step - 1) * right_i - 2 * right_sid) / 2;
        right_sid -= step * right_i;
        right_i -= count;
        let new_cost = left_v + right_v;
        if new_cost > current_cost {
            return current_cost;
        }
        current_cost = new_cost;
        pos = new_pos;
    }
    current_cost
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

    #[test]
    fn part1() {
        let input = super::generate(SAMPLE);
        assert_eq!(37, super::part1(&input));
    }

    #[test]
    fn part2() {
        let input = super::generate(SAMPLE);
        assert_eq!(168, super::part2(&input));
    }
}
