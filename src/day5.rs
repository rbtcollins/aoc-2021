use std::collections::HashMap;

use pathfinding::directed::topological_sort::topological_sort;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator as _};
use winnow::{
    ascii::{dec_uint, newline},
    combinator::{opt, repeat, separated, separated_pair, terminated},
    error::ContextError,
    Parser as _,
};

pub type Pages = u64;

pub struct Puzzle {
    updates: Vec<Vec<usize>>,
    predecessors: [Pages; 64],
    successors: [Pages; 64],
    to_original_values: [u32; 64],
}

pub fn generate(input: &str) -> Puzzle {
    let page = dec_uint::<_, u32, ContextError>;
    let order = terminated(separated_pair(page, "|", page), newline);
    let orders = repeat(1.., order);
    let update_line = separated(1.., page, ",");
    let update = terminated::<_, Vec<u32>, _, _, _, _>(update_line, opt(newline));
    let updates = repeat::<_, _, Vec<Vec<u32>>, _, _>(1.., update);
    let (orderings, updates): (Vec<(u32, u32)>, Vec<Vec<u32>>) =
        separated_pair(orders, newline, updates)
            .parse(input)
            .unwrap();

    let mut to_original_values = [0u32; 64];
    let mut to_new_values = HashMap::new();
    for (a, b) in orderings.iter() {
        if to_new_values.get(a).is_none() {
            let new_value = to_new_values.len() as usize;
            to_new_values.insert(a, new_value);
            to_original_values[new_value as usize] = *a;
        }
        if to_new_values.get(b).is_none() {
            let new_value = to_new_values.len() as usize;
            to_new_values.insert(b, new_value);
            to_original_values[new_value as usize] = *b;
        }
    }

    let updates = updates
        .iter()
        .map(|update| {
            update
                .iter()
                .map(|page| to_new_values.get(page).unwrap())
                .copied()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let orderings = orderings
        .iter()
        .map(|&(a, b)| (to_new_values[&a], to_new_values[&b]))
        .collect::<Vec<_>>();

    let mut successors = [0 as Pages; 64];
    let mut predecessors = [0 as Pages; 64];
    for &(a, b) in orderings.iter() {
        successors[a] |= 1 << b;
        predecessors[b] |= 1 << a;
    }

    // remap page values to fit in a u64 for use in bitsets.
    Puzzle {
        // orderings,
        updates,
        successors,
        predecessors,
        to_original_values,
    }
}

#[tracing::instrument(skip_all)]
fn rule_topo_sort(update: &Vec<usize>, puzzle: &Puzzle) -> Vec<usize> {
    let mut rule_pages = update.clone();
    rule_pages.sort_unstable();

    topo_sort(rule_pages, &puzzle.successors)
}

#[tracing::instrument(skip_all)]
fn topo_sort(rule_pages: Vec<usize>, successors: &[Pages; 64]) -> Vec<usize> {
    let mut rule_pages_u64 = 0u64;
    for page in &rule_pages {
        rule_pages_u64 |= 1 << page;
    }
    topological_sort(&rule_pages, |&page| {
        let successor = successors[page];
        (0..64)
            .filter(|&i| successor & (1 << i) != 0 && rule_pages_u64 & (1 << i) != 0)
            .collect::<Vec<_>>()
    })
    .expect("valid topo sort")
}

fn rule_is_sorted(update: &Vec<usize>, puzzle: &Puzzle) -> bool {
    let mut involved: Pages = 0;
    for page in update {
        involved |= 1 << page;
    }
    let mut seen: Pages = 0;
    for page in update {
        let successors = puzzle.successors[*page] & involved;
        if successors & seen != 0 {
            return false;
        }
        seen |= 1 << page;
    }
    return true;
}

fn sorted_update_mid(update: &Vec<usize>, puzzle: &Puzzle) -> Option<u32> {
    if rule_is_sorted(update, &puzzle) {
        None
    } else {
        let mut sorted = update.clone();
        let mut involved: Pages = 0;
        for page in update {
            involved |= 1 << page;
        }
        let mut seen: Pages = 0;
        let mid_point = update.len() / 2;
        for i in 0..=mid_point {
            // element i is sorted if seen | predecessors == seen
            let page = sorted[i];
            let predecessors = puzzle.predecessors[page] & involved;
            if predecessors | seen != seen {
                //swap with first sorted element
                for j in i + 1..sorted.len() {
                    let page = sorted[j];
                    let predecessors = puzzle.predecessors[page] & involved;
                    if predecessors | seen == seen {
                        sorted.swap(i, j);
                        break;
                    }
                }
            }
            seen |= 1 << sorted[i];
        }
        Some(puzzle.to_original_values[sorted[mid_point]])
    }
}

pub fn part_1(puzzle: &Puzzle) -> u32 {
    puzzle
        .updates
        .iter()
        .filter(|&update| rule_is_sorted(update, &puzzle))
        .map(|update| update[update.len() / 2])
        .map(|page| puzzle.to_original_values[page])
        .sum()
}

pub fn part_1_rayon(puzzle: &Puzzle) -> u32 {
    puzzle
        .updates
        .par_iter()
        .filter(|&update| {
            let rule_sort: Vec<usize> = rule_topo_sort(update, &puzzle);
            rule_sort == *update
        })
        .map(|update| update[update.len() / 2])
        .map(|page| puzzle.to_original_values[page])
        .sum()
}

pub fn part_2(puzzle: &Puzzle) -> u32 {
    puzzle
        .updates
        .iter()
        .filter_map(|update| sorted_update_mid(update, puzzle))
        .sum()
}

pub fn part_2_rayon(puzzle: &Puzzle) -> u32 {
    puzzle
        .updates
        .par_iter()
        .filter_map(|update| sorted_update_mid(update, puzzle))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"29|13
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

    #[test]
    fn test_part_1() {
        let input = generate(INPUT);
        assert_eq!(part_1(&input), 143);
    }

    #[test]
    fn test_part_2() {
        let input = generate(INPUT);
        assert_eq!(part_2(&input), 123);
    }
}
