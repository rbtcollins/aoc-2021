use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Data {
    List(Vec<Data>),
    Number(usize),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Packet {
    left: Data,
    right: Data,
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Data::Number(l), Data::Number(r)) => l.partial_cmp(r),
            (Data::List(l), Data::List(r)) => l.partial_cmp(r),
            (Data::Number(l), _) => Data::List(vec![Data::Number(*l)]).partial_cmp(other),
            (_, Data::Number(r)) => self.partial_cmp(&Data::List(vec![Data::Number(*r)])),
        }
    }
}
impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn generate(input: &str) -> Vec<Packet> {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut x| {
            let left = x.next().unwrap();
            let left: Data = serde_json::from_str(left).unwrap();
            let right = x.next().unwrap();
            let right: Data = serde_json::from_str(right).unwrap();
            Packet { left, right }
        })
        .collect::<Vec<_>>()
}

pub fn part_1(input: &[Packet]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, p)| if p.left < p.right { Some(i + 1) } else { None })
        .sum()
}

pub fn part_2(input: &[Packet]) -> usize {
    let reference = Packet {
        left: Data::List(vec![Data::List(vec![Data::Number(2)])]),
        right: Data::List(vec![Data::List(vec![Data::Number(6)])]),
    };
    [reference.clone()]
        .iter()
        .chain(input.iter())
        .flat_map(|p| [p.left.clone(), p.right.clone()])
        .sorted()
        .enumerate()
        .filter_map(|(i, d)| {
            if d == reference.left || d == reference.right {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn parse() {
        use super::Data;
        let p: Data = serde_json::from_str("[1,2,3,4,5]").unwrap();
        assert_eq!(
            p,
            Data::List(vec![
                Data::Number(1),
                Data::Number(2),
                Data::Number(3),
                Data::Number(4),
                Data::Number(5)
            ])
        );
        let p: Data = serde_json::from_str("[[1],4]").unwrap();
        assert_eq!(
            p,
            Data::List(vec![Data::List(vec![Data::Number(1)]), Data::Number(4),])
        );
    }

    #[test]
    fn part_1() {
        let input = super::generate(SAMPLE);
        assert_eq!(13, super::part_1(&input));
    }

    #[test]
    fn part_2() {
        let input = super::generate(SAMPLE);
        assert_eq!(140, super::part_2(&input));
    }
}
