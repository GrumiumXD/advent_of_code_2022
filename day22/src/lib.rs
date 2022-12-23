use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Floor,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Dir {
    fn turn(&self, m: &Move) -> Self {
        match (self, m) {
            (Dir::Right, Move::Left) => Dir::Up,
            (Dir::Right, Move::Right) => Dir::Down,
            (Dir::Down, Move::Left) => Dir::Right,
            (Dir::Down, Move::Right) => Dir::Left,
            (Dir::Left, Move::Left) => Dir::Down,
            (Dir::Left, Move::Right) => Dir::Up,
            (Dir::Up, Move::Left) => Dir::Left,
            (Dir::Up, Move::Right) => Dir::Right,
            (x, Move::Walk(_)) => *x,
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    height: usize,
}

impl Map {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Self {
        Map {
            height: tiles.len(),
            tiles,
        }
    }

    pub fn get_start(&self) -> (isize, isize) {
        let pos = (&self.tiles[0])
            .iter()
            .position(|t| *t == Tile::Floor)
            .unwrap();

        (pos as isize, 0)
    }

    pub fn get(&self, x: isize, y: isize) -> Tile {
        if y < 0 || y as usize >= self.height {
            return Tile::None;
        }

        let row = &self.tiles[y as usize];

        if x < 0 || x as usize >= row.len() {
            return Tile::None;
        }

        row[x as usize]
    }

    pub fn wrap(&self, x: isize, y: isize, dir: Dir) -> (isize, isize) {
        let mut current = (x, y);
        loop {
            // move in the opposite direction
            let next = match dir {
                Dir::Right => (current.0 - 1, current.1),
                Dir::Down => (current.0, current.1 - 1),
                Dir::Left => (current.0 + 1, current.1),
                Dir::Up => (current.0, current.1 + 1),
            };

            // until a "None" is encountered
            if self.get(next.0, next.1) == Tile::None {
                break;
            }

            current = next;
        }

        current
    }
}

struct Cube {
    map: Map,
    size: isize,
    lookup: HashMap<(isize, isize, Dir), (isize, isize, Dir)>,
}

impl Cube {
    pub fn new(
        map: Map,
        size: isize,
        lookup: HashMap<(isize, isize, Dir), (isize, isize, Dir)>,
    ) -> Self {
        Cube { map, size, lookup }
    }

    pub fn get_start(&self) -> (isize, isize) {
        self.map.get_start()
    }

    pub fn get(&self, x: isize, y: isize) -> Tile {
        self.map.get(x, y)
    }

    pub fn wrap(&self, x: isize, y: isize, dir: Dir) -> (isize, isize, Dir) {
        let m = self.size;

        let x_id = x / m;
        let y_id = y / m;

        if let Some(&(tx, ty, dir_target)) = self.lookup.get(&(x_id, y_id, dir)) {
            use Dir::*;
            let (new_x, new_y) = match (dir, dir_target) {
                (Right, Right) => (tx * m, ty * m + (y % m)),
                (Right, Down) => {
                    let d = (m - 1) - (y % m);
                    (tx * m + d, ty * m)
                }
                (Right, Left) => {
                    let d = (m - 1) - (y % m);
                    ((tx + 1) * m - 1, ty * m + d)
                }
                (Right, Up) => (tx * m + (y % m), (ty + 1) * m - 1),

                (Down, Right) => {
                    let d = (m - 1) - (x % m);
                    (tx * m, ty * m + d)
                }
                (Down, Down) => (tx * m + (x % m), ty * m),
                (Down, Left) => ((tx + 1) * m - 1, ty * m + (x % m)),
                (Down, Up) => {
                    let d = (m - 1) - (x % m);
                    (tx * m + d, (ty + 1) * m - 1)
                }

                (Left, Right) => {
                    let d = (m - 1) - (y % m);
                    (tx * m, ty * m + d)
                }
                (Left, Down) => (tx * m + (y % m), ty * m),
                (Left, Left) => ((tx + 1) * m - 1, ty * m + (y % m)),
                (Left, Up) => {
                    let d = (m - 1) - (y % m);
                    (tx * m + d, (ty + 1) * m - 1)
                }

                (Up, Right) => (tx * m, ty * m + (x % m)),
                (Up, Down) => {
                    let d = (m - 1) - (x % m);
                    (tx * m + d, ty * m)
                }
                (Up, Left) => {
                    let d = (m - 1) - (x % m);
                    ((tx + 1) * m - 1, ty * m + d)
                }
                (Up, Up) => (tx * m + (x % m), (ty + 1) * m - 1),
            };
            return (new_x, new_y, dir_target);
        } else {
            panic!("missing lookup ({}, {}, {:?})", x_id, y_id, dir);
        }
    }
}

#[derive(Debug)]
enum Move {
    Walk(usize),
    Left,
    Right,
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    separated_list1(
        line_ending,
        many1(alt((
            tag(" ").map(|_| Tile::None),
            tag("#").map(|_| Tile::Wall),
            tag(".").map(|_| Tile::Floor),
        ))),
    )
    .map(|v| Map::new(v))
    .parse(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        tag("L").map(|_| Move::Left),
        tag("R").map(|_| Move::Right),
        complete::u64.map(|n| Move::Walk(n as usize)),
    )))(input)
}

fn parse_input(input: &str) -> IResult<&str, (Map, Vec<Move>)> {
    separated_pair(parse_map, tag("\n\n"), parse_moves)(input)
}

pub fn puzzle_1(input: &str) -> String {
    let (_, (map, moves)) = parse_input(input).unwrap();

    let mut dir = Dir::Right;
    let mut current = map.get_start();

    for m in moves.iter() {
        if let Move::Walk(step) = m {
            for _ in 0..*step {
                // go a step in the direction
                let (nx, ny) = match dir {
                    Dir::Right => (current.0 + 1, current.1),
                    Dir::Down => (current.0, current.1 + 1),
                    Dir::Left => (current.0 - 1, current.1),
                    Dir::Up => (current.0, current.1 - 1),
                };

                // check the tile
                match map.get(nx, ny) {
                    Tile::Wall => break,
                    Tile::Floor => current = (nx, ny),
                    Tile::None => {
                        // wrap around
                        let (nx, ny) = map.wrap(current.0, current.1, dir);
                        // and check again
                        if map.get(nx, ny) == Tile::Wall {
                            break;
                        } else {
                            current = (nx, ny);
                        }
                    }
                }
            }
        } else {
            dir = dir.turn(m);
        }
    }

    let result = (current.1 + 1) * 1000 + (current.0 + 1) * 4 + dir as isize;

    result.to_string()
}

pub fn puzzle_2(
    input: &str,
    cube_size: isize,
    lookup: HashMap<(isize, isize, Dir), (isize, isize, Dir)>,
) -> String {
    let (_, (map, moves)) = parse_input(input).unwrap();

    let cube = Cube::new(map, cube_size, lookup);

    let mut dir = Dir::Right;
    let mut current = cube.get_start();

    for m in moves.iter() {
        if let Move::Walk(step) = m {
            for _ in 0..*step {
                // go a step in the direction
                let (nx, ny) = match dir {
                    Dir::Right => (current.0 + 1, current.1),
                    Dir::Down => (current.0, current.1 + 1),
                    Dir::Left => (current.0 - 1, current.1),
                    Dir::Up => (current.0, current.1 - 1),
                };

                // check the tile
                match cube.get(nx, ny) {
                    Tile::Wall => break,
                    Tile::Floor => current = (nx, ny),
                    Tile::None => {
                        // wrap around
                        let (nx, ny, nd) = cube.wrap(current.0, current.1, dir);
                        // and check again
                        if cube.get(nx, ny) == Tile::Wall {
                            break;
                        } else {
                            current = (nx, ny);
                            dir = nd;
                        }
                    }
                }
            }
        } else {
            dir = dir.turn(m);
        }
    }

    let result = (current.1 + 1) * 1000 + (current.0 + 1) * 4 + dir as isize;

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("testinput.txt");

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "6032");
    }

    #[test]
    fn p2() {
        use Dir::*;

        let cube_size = 4;
        let lookup = HashMap::from([
            ((2, 0, Left), (1, 1, Down)),
            ((2, 0, Up), (0, 1, Down)),
            ((2, 0, Right), (3, 2, Left)),
            ((2, 1, Right), (3, 2, Down)),
            ((3, 2, Up), (2, 1, Left)),
            ((3, 2, Right), (2, 0, Left)),
            ((3, 2, Down), (0, 1, Right)),
            ((2, 2, Down), (0, 1, Up)),
            ((2, 2, Left), (1, 1, Up)),
            ((1, 1, Down), (2, 2, Right)),
            ((0, 1, Down), (2, 2, Up)),
            ((0, 1, Left), (3, 2, Up)),
            ((0, 1, Up), (2, 0, Down)),
            ((1, 1, Up), (2, 0, Right)),
        ]);

        let result = puzzle_2(INPUT, cube_size, lookup);
        assert_eq!(result, "5031");
    }
}
