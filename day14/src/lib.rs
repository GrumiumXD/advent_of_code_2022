use std::{collections::HashMap, ops::RangeInclusive};

enum Tile {
    Rock,
    Sand,
}

struct Cave {
    tiles: HashMap<(i32, i32), Tile>,
    lowest: i32,
}

fn parse_ranges(input: &str) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    input
        .lines()
        .flat_map(|line| {
            // parse pairs of coordinats
            let coords = line
                .split(" -> ")
                .map(|pair| {
                    let p = pair
                        .split(',')
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    (p[0], p[1])
                })
                .collect::<Vec<(i32, i32)>>();

            // convert the coordinates into ranges for x and y
            let ranges = coords
                .windows(2)
                .map(|pair| {
                    let mut x = vec![pair[0].0, pair[1].0];
                    x.sort();

                    let mut y = vec![pair[0].1, pair[1].1];
                    y.sort();

                    (x[0]..=x[1], y[0]..=y[1])
                })
                .collect::<Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>>();

            ranges
        })
        .collect::<Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>>()
}

fn parse_cave(input: &str) -> Cave {
    let mut tiles = HashMap::new();
    let mut lowest = 0;

    let ranges = parse_ranges(input);

    for range in ranges.iter() {
        for x in range.0.clone() {
            for y in range.1.clone() {
                tiles.insert((x, y), Tile::Rock);
            }
        }

        if *range.1.end() > lowest {
            lowest = *range.1.end();
        }
    }

    Cave { tiles, lowest }
}

pub fn puzzle_1(input: &str) -> String {
    let mut cave = parse_cave(input);

    // sand filling
    'outer: loop {
        let mut pos = (500, 0);

        // falling
        'inner: loop {
            if let None = cave.tiles.get(&(pos.0, pos.1 + 1)) {
                //down
                pos = (pos.0, pos.1 + 1);
            } else if let None = cave.tiles.get(&(pos.0 - 1, pos.1 + 1)) {
                // down left
                pos = (pos.0 - 1, pos.1 + 1);
            } else if let None = cave.tiles.get(&(pos.0 + 1, pos.1 + 1)) {
                // down right
                pos = (pos.0 + 1, pos.1 + 1);
            } else {
                // resting position
                cave.tiles.insert(pos, Tile::Sand);
                break 'inner;
            }

            if pos.1 > cave.lowest {
                // falling in the void
                break 'outer;
            }
        }
    }

    let sand_count = cave
        .tiles
        .values()
        .filter(|t| match t {
            Tile::Rock => false,
            Tile::Sand => true,
        })
        .count();

    sand_count.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut cave = parse_cave(input);

    let floor = cave.lowest + 2;

    // sand filling
    'outer: loop {
        let mut pos = (500, 0);

        // falling
        'inner: loop {
            if let None = cave.tiles.get(&(pos.0, pos.1 + 1)) {
                //down
                pos = (pos.0, pos.1 + 1);
            } else if let None = cave.tiles.get(&(pos.0 - 1, pos.1 + 1)) {
                // down left
                pos = (pos.0 - 1, pos.1 + 1);
            } else if let None = cave.tiles.get(&(pos.0 + 1, pos.1 + 1)) {
                // down right
                pos = (pos.0 + 1, pos.1 + 1);
            } else {
                // resting position
                cave.tiles.insert(pos, Tile::Sand);

                if pos.0 == 500 && pos.1 == 0 {
                    break 'outer;
                } else {
                    break 'inner;
                }
            }

            if pos.1 == floor - 1 {
                // landing on the floor
                cave.tiles.insert(pos, Tile::Sand);
                break 'inner;
            }
        }
    }

    let sand_count = cave
        .tiles
        .values()
        .filter(|t| match t {
            Tile::Rock => false,
            Tile::Sand => true,
        })
        .count();

    sand_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "24");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "93");
    }
}
