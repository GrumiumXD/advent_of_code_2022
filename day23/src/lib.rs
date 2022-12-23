use std::collections::{HashMap, HashSet};

use num_enum::FromPrimitive;

fn parse_input(input: &str) -> HashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    return Some((x as i32, y as i32));
                }
                None
            })
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
enum Direction {
    #[num_enum(default)]
    North = 0,
    South = 1,
    West = 2,
    East = 3,
}

fn get_neighbors(pos: &(i32, i32), dir: Option<Direction>) -> Vec<(i32, i32)> {
    let (x, y) = pos;

    match dir {
        Some(Direction::North) => vec![(*x - 1, *y - 1), (*x, *y - 1), (*x + 1, *y - 1)],
        Some(Direction::South) => vec![(*x - 1, *y + 1), (*x, *y + 1), (*x + 1, *y + 1)],
        Some(Direction::West) => vec![(*x - 1, *y - 1), (*x - 1, *y), (*x - 1, *y + 1)],
        Some(Direction::East) => vec![(*x + 1, *y - 1), (*x + 1, *y), (*x + 1, *y + 1)],
        None => vec![
            (*x - 1, *y - 1),
            (*x, *y - 1),
            (*x + 1, *y - 1),
            (*x + 1, *y),
            (*x + 1, *y + 1),
            (*x, *y + 1),
            (*x - 1, *y + 1),
            (*x - 1, *y),
        ],
    }
}

pub fn puzzle_1(input: &str) -> String {
    let mut elves = parse_input(input);

    for round in 0..10 {
        // the 4 direction checks for this round
        let checks = (round..(round + 4))
            .into_iter()
            .map(|i| Direction::from(i % 4))
            .collect::<Vec<_>>();

        // data structure for the proposals
        let mut proposals = HashMap::new();

        for pos in elves.iter() {
            // check the 8 neighbors
            let neighbor_count = get_neighbors(pos, None)
                .iter()
                .filter(|&p| elves.contains(p))
                .count();
            if neighbor_count == 0 {
                continue;
            }

            // go through the directions and check the neighbors there
            for d in checks.iter() {
                let side_count = get_neighbors(pos, Some(*d))
                    .iter()
                    .filter(|&p| elves.contains(p))
                    .count();

                if side_count == 0 {
                    // propose a move in the direction

                    let new_pos = match d {
                        Direction::North => (pos.0, pos.1 - 1),
                        Direction::South => (pos.0, pos.1 + 1),
                        Direction::West => (pos.0 - 1, pos.1),
                        Direction::East => (pos.0 + 1, pos.1),
                    };

                    if proposals.contains_key(&new_pos) {
                        // multiple elves try the same field, so set it to none
                        *proposals.get_mut(&new_pos).unwrap() = None;
                        // dbg!("blub");
                    } else {
                        // add the current pos of the elf to this field
                        proposals.insert(new_pos, Some((pos.0, pos.1)));
                    }

                    break;
                }
            }
        }

        // move the elves
        for (target, source) in proposals.iter() {
            if let Some(src) = source {
                // remove the old elf's position
                elves.remove(src);

                // add the new position
                elves.insert((target.0, target.1));
            }
        }
    }

    // find the bounding box
    let x_min = elves.iter().map(|p| p.0).min().unwrap();
    let x_max = elves.iter().map(|p| p.0).max().unwrap();
    let y_min = elves.iter().map(|p| p.1).min().unwrap();
    let y_max = elves.iter().map(|p| p.1).max().unwrap();

    let free_spaces = (x_max - x_min + 1) * (y_max - y_min + 1) - elves.len() as i32;

    free_spaces.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut elves = parse_input(input);

    let mut no_move_round = 0;

    for round in 0.. {
        // the 4 direction checks for this round
        let checks = (round..(round + 4))
            .into_iter()
            .map(|i| Direction::from(i % 4))
            .collect::<Vec<_>>();

        // data structure for the proposals
        let mut proposals = HashMap::new();

        for pos in elves.iter() {
            // check the 8 neighbors
            let neighbor_count = get_neighbors(pos, None)
                .iter()
                .filter(|&p| elves.contains(p))
                .count();
            if neighbor_count == 0 {
                continue;
            }

            // go through the directions and check the neighbors there
            for d in checks.iter() {
                let side_count = get_neighbors(pos, Some(*d))
                    .iter()
                    .filter(|&p| elves.contains(p))
                    .count();

                if side_count == 0 {
                    // propose a move in the direction

                    let new_pos = match d {
                        Direction::North => (pos.0, pos.1 - 1),
                        Direction::South => (pos.0, pos.1 + 1),
                        Direction::West => (pos.0 - 1, pos.1),
                        Direction::East => (pos.0 + 1, pos.1),
                    };

                    if proposals.contains_key(&new_pos) {
                        // multiple elves try the same field, so set it to none
                        *proposals.get_mut(&new_pos).unwrap() = None;
                        // dbg!("blub");
                    } else {
                        // add the current pos of the elf to this field
                        proposals.insert(new_pos, Some((pos.0, pos.1)));
                    }

                    break;
                }
            }
        }

        let mut moved = false;

        // move the elves
        for (target, source) in proposals.iter() {
            if let Some(src) = source {
                // remove the old elf's position
                elves.remove(src);

                // add the new position
                elves.insert((target.0, target.1));

                if !moved {
                    moved = true;
                }
            }
        }

        if !moved {
            no_move_round = round + 1;
            break;
        }
    }

    no_move_round.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "110");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "20");
    }
}
