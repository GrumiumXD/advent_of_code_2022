use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use cached::proc_macro::cached;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

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

fn search(
    valves: &HashMap<String, Valve>,
    current_value: u32,
    current_valve: &str,
    minutes: u32,
    closed_valves: &mut BTreeSet<&str>,
) -> u32 {
    let mut result = current_value;

    if valves.len() != closed_valves.len() {
        let paths = valves[current_valve].paths.clone();

        let best = paths
            .iter()
            .map(|(p, d)| {
                // skip closed valves and not reachable
                if closed_valves.contains(p.as_str()) || *d + 1 > minutes {
                    return current_value;
                }
                let new_minutes = minutes - d - 1;
                // add the new valve to the closed ones
                let mut new_closed = closed_valves.clone();
                new_closed.insert(p.as_str());
                // get the flow rate for the closed one
                let flow = valves[p].flow;

                // recursevily calculate the value for the newly closed valve
                let new_value = search(
                    valves,
                    current_value + new_minutes * flow,
                    p,
                    new_minutes,
                    &mut new_closed,
                );

                new_value
            })
            .max();

        // pick the best best path
        if let Some(b) = best {
            result = result.max(b);
        }
    }
    result
}

#[cached(
    key = "String",
    convert = r#"{        
        let start = "".to_string();
        format!("{}-{}-{}-{}-{}-{}", current_value, me.0, me.1, elephant.0, elephant.1, closed_valves.iter().fold(start, |acc, &curr| {
                format!("{}{},", acc, curr)
            })
        )
    }"#
)]
fn search_elephant(
    valves: &HashMap<String, Valve>,
    current_value: u32,
    me: (&str, u32),
    elephant: (&str, u32),
    closed_valves: &mut BTreeSet<&str>,
) -> u32 {
    let small = if me.1 < elephant.1 { &me } else { &elephant };
    let big = if me.1 >= elephant.1 { &me } else { &elephant };

    let mut result = current_value;

    if valves.len() > closed_valves.len() {
        let my_paths = valves[small.0].paths.clone();
        let elephants_paths = valves[big.0].paths.clone();

        let my_best = my_paths
            .iter()
            .map(|(p, d)| {
                // skip closed valves and not reachable
                if closed_valves.contains(p.as_str()) || *d + 1 > small.1 {
                    return current_value;
                }
                let new_minutes = small.1 - d - 1;
                // add the new valve to the closed ones
                let mut new_closed = closed_valves.clone();
                new_closed.insert(p.as_str());
                // get the flow rate for the closed one
                let flow = valves[p].flow;

                // recursevily calculate the value for the newly closed valve
                let new_value = search_elephant(
                    valves,
                    current_value + new_minutes * flow,
                    (p, new_minutes),
                    (big.0, big.1),
                    &mut new_closed,
                );

                new_value
            })
            .max();

        let elephants_best = elephants_paths
            .iter()
            .map(|(p, d)| {
                // skip closed valves and not reachable
                if closed_valves.contains(p.as_str()) || *d + 1 > big.1 {
                    return current_value;
                }
                let new_minutes = big.1 - d - 1;
                // add the new valve to the closed ones
                let mut new_closed = closed_valves.clone();
                new_closed.insert(p.as_str());
                // get the flow rate for the closed one
                let flow = valves[p].flow;

                // recursevily calculate the value for the newly closed valve
                let new_value = search_elephant(
                    valves,
                    current_value + new_minutes * flow,
                    (small.0, small.1),
                    (p, new_minutes),
                    &mut new_closed,
                );

                new_value
            })
            .max();

        result = match (my_best, elephants_best) {
            (None, None) => result,
            (None, Some(e)) => e,
            (Some(m), None) => m,
            (Some(m), Some(e)) => m.max(e),
        }
    }

    result
}

pub fn puzzle_1(input: &str) -> String {
    let valves = build_graph(input);

    let mut closed = BTreeSet::from(["AA"]);
    let result = search(&valves, 0, "AA", 30, &mut closed);

    result.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let valves = build_graph(input);

    let mut closed = BTreeSet::from(["AA"]);
    let result = search_elephant(&valves, 0, ("AA", 26), ("AA", 26), &mut closed);

    result.to_string()
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
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "1707");
    }
}
