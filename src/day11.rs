use std::{fmt::Display, mem};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum OldOrConstant {
    Old,
    Constant(usize),
}

impl OldOrConstant {
    fn get(&self, old: usize) -> usize {
        match self {
            OldOrConstant::Old => old,
            OldOrConstant::Constant(v) => *v,
        }
    }
}

impl Display for OldOrConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OldOrConstant::Constant(v) => write!(f, "{v}"),
            &OldOrConstant::Old => write!(f, "itself"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Multiply(OldOrConstant, OldOrConstant),
    Sum(OldOrConstant, OldOrConstant),
}

impl Operation {
    fn perform(&self, old: usize) -> usize {
        match self {
            Operation::Multiply(l, r) => l.get(old) * r.get(old),
            Operation::Sum(l, r) => l.get(old) + r.get(old),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (op_str, rhs) = match self {
            Operation::Multiply(_, r) => ("multiplied", r),
            Operation::Sum(_, r) => ("increases", r),
        };
        write!(f, "{op_str} by {rhs}")
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test_divisor: usize,
    on_true: usize,
    on_false: usize,
    inspected: usize,
}

impl Default for Monkey {
    fn default() -> Self {
        Self {
            items: Default::default(),
            op: Operation::Multiply(OldOrConstant::Old, OldOrConstant::Old),
            test_divisor: Default::default(),
            on_true: Default::default(),
            on_false: Default::default(),
            inspected: Default::default(),
        }
    }
}

pub fn generate(input: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let mut current_monkey = Monkey::default();
    for l in input.lines().filter(|l| l.len() > 2) {
        let segments: Vec<_> = l.split(':').collect();

        // eprintln!("{segments:?}");
        match segments[0].trim().get(0..4).unwrap() {
            "Monk" => {
                // current_monkey = Monkey::default();
            }
            "Star" => {
                current_monkey.items = segments[1]
                    .split(", ")
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
            }
            "Oper" => {
                let segments: Vec<_> = segments[1].split(' ').collect();

                let left = match segments[3] {
                    "old" => OldOrConstant::Old,
                    v => OldOrConstant::Constant(v.parse().unwrap()),
                };
                let right = match segments[5] {
                    "old" => OldOrConstant::Old,
                    v => OldOrConstant::Constant(v.parse().unwrap()),
                };
                current_monkey.op = match segments[4] {
                    "+" => Operation::Sum(left, right),
                    "*" => Operation::Multiply(left, right),
                    _v => unreachable!("op {_v}"),
                };
            }
            "Test" => {
                current_monkey.test_divisor =
                    segments[1].split(' ').last().unwrap().parse().unwrap();
            }
            "If t" => {
                current_monkey.on_true = segments[1].split(' ').last().unwrap().parse().unwrap();
            }
            "If f" => {
                current_monkey.on_false = segments[1].split(' ').last().unwrap().parse().unwrap();
                monkeys.push(current_monkey);
                current_monkey = Monkey::default();
            }
            _v => unreachable!("outer {_v}"),
        }
    }
    monkeys
}

pub fn part_1(input: &[Monkey]) -> usize {
    pound_monkeys(input, 20, |w| w / 3).1
}

pub fn pound_monkeys<S: Fn(usize) -> usize>(
    input: &[Monkey],
    cycles: usize,
    scale: S,
) -> (Vec<usize>, usize) {
    let mut monkeys = input.to_owned();
    for _round in 0..cycles {
        for monkey_idx in 0..monkeys.len() {
            // eprintln!("Monkey {monkey_idx}:");
            let items = mem::take(&mut monkeys[monkey_idx].items);
            monkeys[monkey_idx].inspected += items.len();
            for worry in items {
                // eprintln!("  Monkey inspects item with a worry level of {worry}.");
                let op = &monkeys[monkey_idx].op;
                let worry = op.perform(worry);
                // eprintln!("    Worry level is {op} to {worry}.");
                let worry = scale(worry);
                // eprintln!(
                //     "    Monkey gets bored with item. Worry level is divided by 3 to {worry}."
                // );
                let test_divisor = monkeys[monkey_idx].test_divisor;
                let target = if worry % test_divisor == 0 {
                    // eprintln!("    Current worry level is divisible by {test_divisor}.");
                    monkeys[monkey_idx].on_true
                } else {
                    // eprintln!("    Current worry level is not divisible by {test_divisor}.");
                    monkeys[monkey_idx].on_false
                };
                // eprintln!("    Item with worry level {worry} is thrown to monkey {target}");
                monkeys[target].items.push(worry);
            }
        }
    }
    let inspections = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();

    let business = inspections
        .iter()
        .sorted_by(|a, b| Ord::cmp(b, a))
        .cloned()
        .collect::<Vec<_>>();
    // eprintln!("{:?}", business);
    (inspections, business.iter().take(2).product())
}

pub fn part_2(input: &[Monkey]) -> usize {
    let divisors = input.iter().map(|m| m.test_divisor).collect::<Vec<_>>();
    let common_factor: usize = divisors.iter().product();
    pound_monkeys(input, 10000, |w| w % common_factor).1

    // what we want is: f such that
    // (f(w) % m1.divisor) == w % m1.divisor &&
    // f(w) % m2.divisor == w % m2.divisor &&
    // ...
    // f(w) % m3.divisor == w % m3.divisor &&
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3
  
  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0
  
  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3
  
  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1
  
"#;

    #[test]
    fn part_1() {
        let input = super::generate(SAMPLE);
        assert_eq!(10605, super::part_1(&input));
    }

    #[test]
    fn part_2() {
        let input = super::generate(SAMPLE);
        assert_eq!(2713310158, super::part_2(&input));
    }

    #[ignore]
    #[test]
    fn part_2_scan() {
        let input = super::generate(SAMPLE);
        let rounds = [
            (1, [2, 4, 3, 6]),
            (20, [99, 97, 8, 103]),
            (1000, [5204, 4792, 199, 5192]),
            (2000, [10419, 9577, 392, 10391]),
            (3000, [15638, 14358, 587, 15593]),
            (4000, [20858, 19138, 780, 20797]),
            (5000, [26075, 23921, 974, 26000]),
            (6000, [31294, 28702, 1165, 31204]),
            (7000, [36508, 33488, 1360, 36400]),
            (8000, [41728, 38268, 1553, 41606]),
            (9000, [46945, 43051, 1746, 46807]),
            (10000, [52166, 47830, 1938, 52013]),
        ];
        for d in 2..(23 * 13 * 19 * 17 + 1) {
            for (cycles, inspections) in rounds {
                let (inspected, _) = super::pound_monkeys(&input, cycles, |w| w % d);
                if inspections[..] != inspected[..] {
                    eprintln!(
                        "Disproved {d} at {cycles} {:?} != {:?}",
                        inspections, inspected
                    );
                    break;
                } else {
                    eprintln!("{d} is a candidate at {cycles}");
                }
            }
        }
    }
}
