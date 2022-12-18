use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::{
    algo::dijkstra,
    graph::{Graph, NodeIndex},
    Undirected,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

// #[derive(Debug)]
// struct Valve {
//     flow: u32,
//     paths: Vec<String>,
// }

#[derive(Debug)]
struct Valve {
    flow: u32,
    paths: HashMap<String, u32>,
}

fn parse_valves(input: &str) -> IResult<&str, Vec<(String, (u32, Vec<String>))>> {
    separated_list0(
        newline,
        separated_pair(
            preceded(
                tag("Valve "),
                separated_pair(alpha1::<&str, _>, tag(" has flow rate="), complete::u32),
            ),
            tag("; "),
            preceded(
                alt((
                    tag("tunnels lead to valves "),
                    tag("tunnel leads to valve "),
                )),
                separated_list0(tag(", "), alpha1),
            ),
        )
        .map(|(p, v)| {
            let paths = v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            let id = p.0.to_string();

            (id, (p.1, paths))
        }),
    )(input)
}

fn build_graph(input: &str) -> HashMap<String, Valve> {
    let (_, valves) = parse_valves(input).unwrap();

    let flows = valves
        .iter()
        .map(|(k, (f, _))| (k.clone(), *f))
        .collect::<HashMap<_, _>>();

    let paths = valves
        .into_iter()
        .map(|(k, (_, p))| (k, p))
        .collect::<HashMap<_, _>>();

    let mut valves = HashMap::new();

    for id in flows.keys() {
        if id != "AA" && flows[id] == 0 {
            continue;
        }

        // let mut dist = HashMap::from([("AA".to_string(), 0)]);
        let mut dist = HashMap::new();
        let mut visited = HashSet::from([id]);
        let mut queue = VecDeque::from([(0, id)]);

        while !queue.is_empty() {
            let (distance, id) = queue.pop_front().unwrap();

            for neighbor in paths[id].iter() {
                if visited.contains(neighbor) {
                    continue;
                }
                visited.insert(neighbor);
                if flows[neighbor] > 0 {
                    // insert dist
                    dist.insert(neighbor.clone(), distance + 1);
                }

                queue.push_back((distance + 1, neighbor));
            }
        }
        valves.insert(
            id.clone(),
            Valve {
                flow: flows[id],
                paths: dist,
            },
        );
    }

    valves
}

// fn parse_tunnels(
//     input: &str,
// ) ->
//     HashMap<String, ValveWithDistance>
// {
//     let (_input, valves) = parse_valves(input).unwrap();

//     let mut lookup = HashMap::new();
//     // add the edges
//     for (idx, v) in valves.iter().enumerate() {
//         let ni = NodeIndex::new(idx);

//         // update the lookup table
//         lookup.insert(v.0.to_string(), (ni, v.1.flow));
//     }

//     let edges = valves
//         .iter()
//         .flat_map(|(k, v)| {
//             let l = &lookup;
//             let src = l[k].0;

//             v.paths.iter().map(move |d| {
//                 let dest = l[d].0;

//                 (src.clone(), dest.clone())
//             })
//         })
//         .collect::<Vec<_>>();

//     let graph = Graph::<(), u32, Undirected>::from_edges(edges);

//     let costs = graph
//         .node_indices()
//         .map(|idx| (idx, dijkstra(&graph, idx, None, |_| 1 as u32)))
//         .collect::<HashMap<_, _>>();

//     let final = lookup.into_iter().filter_map(|(k, v)| {

//     });

//     // (lookup, graph)
// }

// fn check_path(
//     valves: &HashMap<String, Valve>,
//     useless: &Vec<String>,
//     current_node: &str,
//     current_release: u32,
//     minutes: u32,
// ) -> u32 {
//     if minutes == 0 {
//         return current_release;
//     }
//     if useless.len() == valves.len() {
//         return current_release;
//     }

//     let path_count = valves[current_node].paths.len();
//     let range = if !useless.iter().any(|s| s == current_node) {
//         0..path_count + 1
//     } else {
//         0..path_count
//     };

//     range
//         .map(|i| {
//             if i == path_count {
//                 // open the current valve
//                 let mut useless = useless.clone();
//                 useless.push(current_node.to_string());

//                 let current_release = current_release + (valves[current_node].flow * (minutes - 1));
//                 return check_path(valves, &useless, current_node, current_release, minutes - 1);
//             }
//             // go down the path
//             let current_node = valves[current_node].paths[i].as_str();
//             check_path(valves, useless, current_node, current_release, minutes - 1)
//         })
//         .max()
//         .unwrap()
// }

fn search(valves: &HashMap<String, Valve>, minutes: u32, current: &String, opened: Vec<&String>) {
    todo!()
}

pub fn puzzle_1(input: &str) -> String {
    let valves = build_graph(input);

    // let (lookup, graph) = parse_tunnels(input);

    // let mut to_open = lookup
    //     .iter()
    //     .filter_map(|(k, v)| {
    //         if v.1 > 0 {
    //             return Some(k.as_str());
    //         }

    //         None
    //     })
    //     .collect::<Vec<_>>();

    // let mut current = "AA";
    // let mut minutes = 30;
    // let mut pressure = 0;

    // // get all shortest paths
    // let costs = graph
    //     .node_indices()
    //     .map(|idx| (idx, dijkstra(&graph, idx, None, |_| 1 as u32)))
    //     .collect::<HashMap<_, _>>();

    // loop {
    //     dbg!(current);
    //     let node = lookup[current].0;

    //     let current_costs = &costs[&node];

    //     let values = to_open
    //         .iter()
    //         .filter_map(|id| {
    //             let n = lookup[*id];
    //             let path = current_costs[&n.0];
    //             let flow = n.1;

    //             let minute_costs = path + 1;
    //             if minute_costs >= minutes {
    //                 return None;
    //             }
    //             let released_pressure = (minutes - minute_costs) * flow;

    //             Some((*id, minute_costs, released_pressure))
    //         })
    //         .inspect(|v| {
    //             dbg!(v);
    //         });

    //     let best_option = values.max_by(|a, b| a.2.cmp(&b.2));

    //     dbg!(best_option);

    //     if let Some(target) = best_option {
    //         minutes -= target.1;
    //         pressure += target.2;
    //         to_open.retain(|s| *s != target.0);
    //         current = target.0;
    //     } else {
    //         // no possible path exists anymore
    //         break;
    //     }
    // }

    // let (_input, valves) = parse_valves(input).unwrap();

    // // collect all vales in a hash map
    // let valves = valves.into_iter().collect::<HashMap<_, _>>();

    // let useless_valves = valves
    //     .iter()
    //     .filter_map(|(k, v)| {
    //         if v.flow == 0 {
    //             return Some(k.clone());
    //         }

    //         None
    //     })
    //     .collect::<Vec<String>>();

    // // dbg!(valves);
    // // dbg!(useless_valves);

    // let max_release = check_path(&valves, &useless_valves, "AA", 0, 30);

    // max_release.to_string()

    "abc".to_string()
}

pub fn puzzle_2(input: &str) -> String {
    "abc".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "1651");
    }

    #[test]
    #[ignore]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "56000011");
    }
}
