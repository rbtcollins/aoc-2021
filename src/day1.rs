use std::collections::HashMap;

use winnow::combinator::repeat;
use winnow::{
    ascii::space1,
    combinator::{opt, seq, terminated},
};
use winnow::{
    ascii::{dec_uint, line_ending},
    error::ContextError,
    prelude::*,
};

pub fn generate(input: &str) -> Vec<(u32, u32)> {
    repeat(
        0..,
        terminated(
            seq!(
                dec_uint::<_, _, ContextError>,
                _: space1,
                dec_uint

            ),
            opt(line_ending),
        ),
    )
    .parse(input)
    .unwrap()
}

pub fn part_1(input: &[(u32, u32)]) -> u32 {
    let (mut l, mut r): (Vec<_>, Vec<_>) = input.iter().copied().unzip();
    l.sort_unstable();
    r.sort_unstable();
    l.into_iter()
        .zip(r.into_iter())
        .map(|(a, b)| (a as i32 - b as i32).abs() as u32)
        .sum()
}

pub fn part_2(input: &[(u32, u32)]) -> u32 {
    let (l, mut r): (Vec<_>, Vec<_>) = input.iter().copied().unzip();
    r.sort_unstable();
    let counts = r
        .chunk_by(PartialEq::eq)
        .into_iter()
        .map(|v| (v[0], v.len() as u32))
        .collect::<HashMap<_, _>>();
    l.into_iter()
        .map(|v| counts.get(&v).unwrap_or(&0) * v)
        .sum()
}
