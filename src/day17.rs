#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Rocks {
    Line,
    Cross,
    Bracket,
    I,
    Box,
}

impl Rocks {
    fn collide(&self, x:usize, y:usize, height:usize, chamber: &[[bool;7]]) -> bool {
        match self {
            Rocks::Line => {x < 1 || x>7 || chamber[height][x]},
            Rocks::Cross => todo!(),
            Rocks::Bracket => todo!(),
            Rocks::I => todo!(),
            Rocks::Box => todo!(),
        }
    }
}

pub fn generate(input: &str) -> Vec<Direction> {
    input
        .as_bytes()
        .filter_map(|b| match b {
            b'<' => Some(Left),
            b'>' => Some(Right),
            _ => None,
        })
        .collect()
}

pub fn part_1(input: &[Direction]) -> usize {
    let mut chamber = [[false; 7]; 2022 * 4 + 3];
    // the bottom is rock
    chamber[0] = [true; 7];

    let rock_order = [
        Rocks::Line,
        Rocks::Cross,
        Rocks::Bracket,
        Rocks::I,
        Rocks::Box,
    ]
    .iter()
    .cycle();
    
    for (r, index) in rock_order.zip(0..2022) {

    }

    0
}

// pub fn part_2(input: &[(&str, i64, Vec<&str>)]) -> usize {
//     let g_useful = build_graph(input);
//     max_flow(&g_useful, [(26, "AA"), (26, "AA")], vec!["AA"]).1 as usize
// }

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
"#;

    #[test]
    fn part_1() {
        let input = super::generate(SAMPLE);
        assert_eq!(3068, super::part_1(&input));
    }

    // #[test]
    // fn part_2() {
    //     let input = super::generate(SAMPLE);
    //     assert_eq!(1707, super::part_2(&input));
    // }
}
