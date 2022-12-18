use std::mem;

use petgraph::{algo::dijkstra, prelude::DiGraphMap};

peg::parser! {
  grammar sensor_parser() for str {
    rule number() -> i64
      = n:$("-"? ['0'..='9']+) {? n.parse().or(Err("i64")) }

    pub rule newline()
      = ['\r' | '\n' ]*<1,2>

    pub rule valve() -> &'input str
      = v:$(['A'..='Z' | 'a'..='z']+) {v}

    pub rule oneline() -> (&'input str, i64, Vec<&'input str>)
      = "Valve " v:valve() " has flow rate=" flow:number() "; tunnel" "s"? " lead" "s"? " to valve" "s"? " " vs:valve() ** ", " {(v, flow, vs)
    }

    pub rule valves() -> Vec<(&'input str, i64, Vec<&'input str>)>
     = i:oneline() ** newline() newline()? {i}
  }
}

pub fn generate(input: &str) -> Vec<(&str, i64, Vec<&str>)> {
    sensor_parser::valves(input).unwrap()
}

pub fn part_1(input: &[(&str, i64, Vec<&str>)]) -> usize {
    let g_useful = build_graph(input);

    // full set of orders is 15! trim in following way: only generate a proposed
    // path while the 30 has not been exceeded. Heuristics like taking the most
    // impactful (== largest flow for smallest expenditure) can help, but are
    // vulnerable to local minima. use a recursive implementation: figuring out
    // cutting in S-J-T was too much for my small brain.
    max_flow(&g_useful, [(30, "AA")], vec!["AA"]).1 as usize
}

pub fn part_2(input: &[(&str, i64, Vec<&str>)]) -> usize {
    let g_useful = build_graph(input);
    max_flow(&g_useful, [(26, "AA"), (26, "AA")], vec!["AA"]).1 as usize
}

pub fn build_graph<'a>(
    input: &'a [(&'a str, i64, Vec<&'a str>)],
) -> DiGraphMap<&'a str, (i64, i64)> {
    let mut g = DiGraphMap::<&str, usize>::new();
    // calculate simple travel costs for the graph from each node to all others
    for (node, _, nodes) in input.iter() {
        for dest in nodes {
            g.add_edge(node, dest, 1);
        }
    }
    let mut useful_nodes = input
        .iter()
        .filter_map(|(n, flow, _)| {
            if *flow != 0 || *n == "AA" {
                Some((*n, flow))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    useful_nodes.sort();
    // for each useful node compute the cost to each other useful node and
    // insert into a new graph
    let mut g_useful =
        DiGraphMap::<&str, _>::with_capacity(useful_nodes.len(), useful_nodes.len() * 2);

    // eprintln!("useful {useful_nodes:?}");

    for (node, _flow) in useful_nodes.iter() {
        let node_costs = dijkstra(&g, node, None, |_| 1);
        for (dest, dest_flow) in useful_nodes.iter() {
            if node == dest {
                continue;
            }
            let cost = *node_costs.get(*dest).unwrap();
            if cost + 1 > 30 {
                // from node to dest + turning on the tap would exceed the entire time budget.
                continue;
            }
            // cost +1 to include tap turning overheads
            g_useful.add_edge(*node, dest, (cost + 1, **dest_flow));
        }
    }

    let mut useful_nodes = g_useful.nodes().collect::<Vec<_>>();
    useful_nodes.sort();
    g_useful
}

fn max_flow<'a, const N: usize>(
    g: &'a DiGraphMap<&str, (i64, i64)>,
    mut cur_node: [(i64, &'a str); N], // actors node, remaining time
    mut current_stack: Vec<&'a str>,
) -> (Vec<&'a str>, i64) {
    if cur_node.iter().map(|(t, _)| *t).max().unwrap() <= 0 {
        return (current_stack, 0);
    }
    // test breadth-first
    for i in 1..N {
        if cur_node[i].0 > cur_node[0].0 {
            let (base, rest) = cur_node.split_at_mut(1);
            mem::swap(&mut base[0], &mut rest[i - 1]);
        }
    }
    // eprintln!("{cur_node:?}");

    // now we only modify cur_node[0].
    let outbound = g
        .edges(cur_node[0].1)
        .filter(|(_from, n, (_travel_time, _flow))| !current_stack.contains(n))
        .collect::<Vec<_>>();

    // eprintln!("{outbound:?}");
    let mut best_flow = 0;
    let mut tried_any = false;
    for (_from, node, (travel_time, flow)) in outbound {
        let next_remaining_time = cur_node[0].0 - travel_time;

        if next_remaining_time <= 0 {
            // this path cannot continue;
            continue;
        }

        tried_any = true;
        current_stack.push(node);

        let flow_to_end = next_remaining_time * flow;
        let mut next_cur_node = cur_node;
        next_cur_node[0].0 = next_remaining_time;
        next_cur_node[0].1 = node;
        let (returned_current_stack, child_flow) = max_flow(g, next_cur_node, current_stack);
        current_stack = returned_current_stack;
        current_stack.pop();
        best_flow = best_flow.max(child_flow + flow_to_end);
    }
    if !tried_any {
        // all paths took the highest time actor to out of time, mark it as unusable and try other actors
        cur_node[0].0 = -1;
        max_flow(g, cur_node, current_stack)
    } else {
        (current_stack, best_flow)
    }
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

    #[test]
    fn parse() {
        assert_eq!(
            super::sensor_parser::valves(SAMPLE),
            Ok(vec![
                ("AA", 0, vec!["DD", "II", "BB"]),
                ("BB", 13, vec!["CC", "AA"]),
                ("CC", 2, vec!["DD", "BB"]),
                ("DD", 20, vec!["CC", "AA", "EE"]),
                ("EE", 3, vec!["FF", "DD"]),
                ("FF", 0, vec!["EE", "GG"]),
                ("GG", 0, vec!["FF", "HH"]),
                ("HH", 22, vec!["GG"]),
                ("II", 0, vec!["AA", "JJ"]),
                ("JJ", 21, vec!["II"])
            ])
        );
    }

    #[test]
    fn part_1() {
        let input = super::generate(SAMPLE);
        assert_eq!(1651, super::part_1(&input));
    }

    #[test]
    fn part_2() {
        let input = super::generate(SAMPLE);
        assert_eq!(1707, super::part_2(&input));
    }
}
