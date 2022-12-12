use petgraph::{
    algo::dijkstra,
    graph::{Graph, NodeIndex},
    Directed,
};

enum Elevation {
    Start,
    Goal,
    Node(i32),
}

fn parse_graph(input: &str) -> (NodeIndex, NodeIndex, Graph<i32, i32>) {
    let height = input.lines().count();

    let elevations = input
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(|c| {
                    if c == 'S' {
                        return Elevation::Start;
                    } else if c == 'E' {
                        return Elevation::Goal;
                    }

                    let v = (c as i32) - 96;
                    Elevation::Node(v)
                })
                .collect::<Vec<Elevation>>()
        })
        .collect::<Vec<Elevation>>();

    let width = elevations.len() / height;

    let mut start = NodeIndex::new(0);
    let mut goal = NodeIndex::new(0);
    let mut graph = Graph::<i32, i32, Directed>::new();

    // add the nodes
    for el in elevations.iter() {
        match el {
            Elevation::Start => {
                start = graph.add_node(1);
            }
            Elevation::Goal => {
                goal = graph.add_node(26);
            }
            Elevation::Node(v) => {
                graph.add_node(*v);
            }
        }
    }

    // add the edges
    for i in 0..elevations.len() {
        let x = i % width;
        let y = i / width;

        let index = NodeIndex::new(i);
        let value = graph[index];

        // north node
        if y > 0 {
            let n_index = NodeIndex::new(x + (y - 1) * width);
            let n_value = graph[n_index];

            if n_value - value <= 1 {
                graph.add_edge(index, n_index, 1);
            }
        }

        // east node
        if x < width - 1 {
            let e_index = NodeIndex::new(x + 1 + y * width);
            let e_value = graph[e_index];

            if e_value - value <= 1 {
                graph.add_edge(index, e_index, 1);
            }
        }

        // south node
        if y < height - 1 {
            let s_index = NodeIndex::new(x + (y + 1) * width);
            let s_value = graph[s_index];

            if s_value - value <= 1 {
                graph.add_edge(index, s_index, 1);
            }
        }

        // west node
        if x > 0 {
            let w_index = NodeIndex::new(x - 1 + y * width);
            let w_value = graph[w_index];

            if w_value - value <= 1 {
                graph.add_edge(index, w_index, 1);
            }
        }
    }

    (start, goal, graph)
}

pub fn puzzle_1(input: &str) -> String {
    let (start, goal, graph) = parse_graph(input);

    let costs = dijkstra(&graph, start, Some(goal), |_| 1);

    let length = costs[&goal];

    length.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (_, goal, mut graph) = parse_graph(input);

    // reverse the edges and calculate all costs starting from the goal
    graph.reverse();
    let costs = dijkstra(&graph, goal, None, |_| 1);

    // filter out the costs that don't end at elevation 1
    let filtered_costs = costs
        .iter()
        .filter_map(|(idx, cost)| {
            let elevation = graph[*idx];
            if elevation == 1 {
                return Some(*cost);
            }
            None
        })
        .collect::<Vec<i32>>();

    let length = filtered_costs.iter().min().unwrap();

    length.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "31");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "29");
    }
}
