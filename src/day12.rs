use petgraph::{algo::dijkstra, prelude::DiGraphMap};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Map {
    start: Point,
    goal: Point,
    heights: Vec<Vec<usize>>,
    graph: DiGraphMap<Point, usize>,
}

pub fn generate(input: &str) -> Map {
    let mut result = Map::default();

    let heights = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.as_bytes()
                .iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if b'a' <= *c && b'z' >= *c {
                        Some((*c - b'a') as usize)
                    } else if *c == b'S' {
                        result.start = Point { x, y };
                        // eprintln!("start {:?}", result.start);
                        Some(0)
                    } else if *c == b'E' {
                        result.goal = Point { x, y };
                        // eprintln!("goal {:?}", result.goal);
                        Some(25)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let width = heights[0].len();
    // build a graph from the heights.
    let mut g =
        DiGraphMap::<Point, usize>::with_capacity(heights.len() * width, heights.len() * width);
    // visit every point and calculate destinations
    for (y, row) in heights.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let here = Point { x, y };
            let max_elevation = height + 1;
            if y > 0 && heights[y - 1][x] <= max_elevation {
                g.add_edge(Point { x, y: y - 1 }, here, 1);
            }
            if y + 1 < heights.len() && heights[y + 1][x] <= max_elevation {
                g.add_edge(Point { x, y: y + 1 }, here, 1);
            }
            if x > 0 && heights[y][x - 1] <= max_elevation {
                g.add_edge(Point { x: x - 1, y }, here, 1);
            }
            if x + 1 < row.len() && heights[y][x + 1] <= max_elevation {
                g.add_edge(Point { x: x + 1, y }, here, 1);
            }
        }
    }
    result.heights = heights;
    result.graph = g;
    result
}

pub fn part_1(input: &Map) -> usize {
    let node_costs = dijkstra(&input.graph, input.goal, Some(input.start), |_| 1);
    node_costs[&input.start]
}

pub fn part_2(input: &Map) -> usize {
    let node_costs = dijkstra(&input.graph, input.goal, None, |_| 1);
    let mut costs = node_costs.into_iter().collect::<Vec<_>>();
    costs.sort_by(|l, r| l.1.cmp(&r.1));
    *costs
        .iter()
        .filter_map(|(n, c)| {
            if input.heights[n.y][n.x] == 0 {
                Some(c)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    #[test]
    fn gen() {
        let input = super::generate(SAMPLE);
        assert_eq!(input.start, super::Point { x: 0, y: 0 });
        assert_eq!(input.goal, super::Point { x: 5, y: 2 });
    }

    #[test]
    fn part_1() {
        let input = super::generate(SAMPLE);
        assert_eq!(31, super::part_1(&input));
    }

    #[test]
    fn part_2() {
        let input = super::generate(SAMPLE);
        assert_eq!(29, super::part_2(&input));
    }
}
