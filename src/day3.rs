use winnow::{
    ascii::dec_uint,
    combinator::{alt, opt, repeat, separated_foldl1, seq},
    error::ContextError,
    token::any,
    Parser as _,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mul {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Op {
    Mul(Mul),
    Do,
    Dont,
}

pub fn generate(input: &str) -> &str {
    input
}

pub fn parse_ops(input: &str) -> Vec<Option<Op>> {
    let op = alt((
        seq! {
                Mul {
                    _: "mul(",
                    x: dec_uint::<_, u32, ContextError>,
                    _: ",",
                    y: dec_uint::<_, u32, ContextError>,
                    _: ")",
            }
        }
        .map(|m| Some(Op::Mul(m))),
        "do()".map(|_| Some(Op::Do)),
        "don't()".map(|_| Some(Op::Dont)),
        any.map(|_| None),
    ));
    repeat(0.., op).parse(input).unwrap()
}

pub fn part_1(input: &str) -> usize {
    parse_ops(input)
        .into_iter()
        .filter_map(|o| match o {
            Some(Op::Mul(m)) => Some(m),
            _ => None,
        })
        .fold(0, |acc, m| acc + m.x * m.y) as usize
}

pub fn part_2(input: &str) -> usize {
    parse_ops(input)
        .into_iter()
        .filter_map(|o| o)
        .fold((0, true), |(acc, enabled), o| match (enabled, o) {
            (true, Op::Mul(m)) => (acc + m.x * m.y, enabled),
            (_, Op::Do) => (acc, true),
            (_, Op::Dont) => (acc, false),
            _ => (acc, enabled),
        })
        .0 as usize
}

pub fn part_1_fold(input: &str) -> usize {
    let op = alt((
        seq! (
           _: "mul(",
           dec_uint::<_, u32, ContextError>,
           _: ",",
           dec_uint::<_, u32, ContextError>,
           _: ")",
        )
        .map(|(x, y)| x * y),
        opt(any).map(|_| 0),
    ));
    separated_foldl1(op, any, |l, _, r| l + r)
        .parse(input)
        .unwrap() as usize
}

pub fn part_2_fold(_input: &str) -> usize {
    0
    // parse_ops(input)
    //     .into_iter()
    //     .filter_map(|o| o)
    //     .fold((0, true), |(acc, enabled), o| match (enabled, o) {
    //         (true, Op::Mul(m)) => (acc + m.x * m.y, enabled),
    //         (_, Op::Do) => (acc, true),
    //         (_, Op::Dont) => (acc, false),
    //         _ => (acc, enabled),
    //     })
    //     .0 as usize
}

#[cfg(test)]
mod tests {}
