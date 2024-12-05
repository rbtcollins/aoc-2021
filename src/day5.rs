use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::prelude::topological_sort_into_groups;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator as _};
use winnow::{
    ascii::{dec_uint, newline},
    combinator::{opt, repeat, separated, separated_pair, terminated},
    error::ContextError,
    Parser as _,
};

pub fn generate(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let page = dec_uint::<_, u32, ContextError>;
    let order = terminated(separated_pair(page, "|", page), newline);
    let orders = repeat(1.., order);
    let update_line = separated(1.., page, ",");
    let update = terminated::<_, Vec<u32>, _, _, _, _>(update_line, opt(newline));
    let updates = repeat::<_, _, Vec<Vec<u32>>, _, _>(1.., update);
    separated_pair(orders, newline, updates)
        .parse(input)
        .unwrap()
}

fn rule_topo_sort(update: &Vec<u32>, orderings: &HashSet<(u32, u32)>) -> Vec<u32> {
    let relevant_rules = update
        .iter()
        .tuple_combinations()
        .filter_map(|(&a, &b)| {
            if orderings.contains(&(a, b)) {
                Some((a, b))
            } else if orderings.contains(&(b, a)) {
                Some((b, a))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut rule_pages = relevant_rules
        .iter()
        .flat_map(|(r1, r2)| [r1, r2])
        .copied()
        .collect::<HashSet<u32>>()
        .iter()
        .copied()
        .collect::<Vec<_>>();
    rule_pages.sort_unstable();
    rule_pages.dedup();
    let rule_pages = rule_pages;
    topological_sort_into_groups(&rule_pages, |&page| {
        let succ = relevant_rules
            .iter()
            .filter_map(|(r1, r2)| if *r1 == page { Some(*r2) } else { None })
            .collect::<Vec<_>>();
        succ
    })
    .unwrap()
    .concat()
}

pub fn part_1((orderings, updates): &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    let orderings = orderings.iter().copied().collect::<HashSet<_>>();
    updates
        .iter()
        .filter(|&update| {
            let rule_sort = rule_topo_sort(update, &orderings);
            rule_sort == *update
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn part_1_rayon((orderings, updates): &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    let orderings = orderings.iter().copied().collect::<HashSet<_>>();
    updates
        .par_iter()
        .filter(|&update| {
            let rule_sort = rule_topo_sort(update, &orderings);
            rule_sort == *update
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn part_2((orderings, updates): &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    let orderings = orderings.iter().copied().collect::<HashSet<_>>();
    updates
        .iter()
        .filter_map(|update| {
            let rule_sort = rule_topo_sort(update, &orderings);
            if rule_sort == *update {
                None
            } else {
                Some(rule_sort)
            }
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn part_2_rayon((orderings, updates): &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    let orderings = orderings.iter().copied().collect::<HashSet<_>>();
    updates
        .par_iter()
        .filter_map(|update| {
            let rule_sort = rule_topo_sort(update, &orderings);
            if rule_sort == *update {
                None
            } else {
                Some(rule_sort)
            }
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"29|13
47|13
47|29
47|53
47|61
53|13
53|29
61|13
61|29
61|53
75|13
75|29
75|47
75|53
75|61
97|13
97|29
97|47
97|53
97|61
97|75

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        let input = generate(input);
        assert_eq!(part_1(&input), 143);
    }
}
