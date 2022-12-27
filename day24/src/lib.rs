use std::collections::HashSet;

#[derive(Clone)]
struct Blizzard {
    pos: (i32, i32),
    dir: (i32, i32),
}

#[derive(Clone)]
struct Field {
    blizzards: Vec<Blizzard>,
    width: usize,
    height: usize,
}

impl Field {
    pub fn new(blizzards: Vec<Blizzard>, width: usize, height: usize) -> Self {
        Field {
            blizzards,
            width,
            height,
        }
    }

    pub fn move_blizzards(&mut self) {
        for b in self.blizzards.iter_mut() {
            let mut new_pos = (b.pos.0 + b.dir.0, b.pos.1 + b.dir.1);
            if new_pos.0 == 0 {
                new_pos.0 = (self.width - 2) as i32;
            } else if new_pos.0 == (self.width - 1) as i32 {
                new_pos.0 = 1;
            } else if new_pos.1 == 0 {
                new_pos.1 = (self.height - 2) as i32;
            } else if new_pos.1 == (self.height - 1) as i32 {
                new_pos.1 = 1;
            }

            b.pos = new_pos;
        }
    }

    pub fn get_goal(&self) -> (i32, i32) {
        ((self.width - 2) as i32, (self.height - 1) as i32)
    }

    pub fn neighbors(&self, pos: &(i32, i32)) -> Vec<(i32, i32)> {
        let positions = vec![
            (pos.0, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ];

        let start = (1, 0);
        let goal = self.get_goal();

        positions
            .iter()
            .filter_map(|p| {
                if start.eq(p) || goal.eq(p) {
                    return Some((p.0, p.1));
                }
                // remove the border and outer positions
                if p.0 == 0
                    || p.0 == (self.width - 1) as i32
                    || p.1 <= 0
                    || p.1 >= (self.height - 1) as i32
                {
                    return None;
                }

                Some((p.0, p.1))
            })
            .collect::<Vec<_>>()
    }

    pub fn free_spaces(&self) -> Vec<(i32, i32)> {
        let occupied = self
            .blizzards
            .iter()
            .map(|b| (b.pos.0, b.pos.1))
            .collect::<Vec<_>>();

        let mut spaces = vec![(1, 0), self.get_goal()];
        for x in 1..(self.width - 1) {
            for y in 1..(self.height - 1) {
                let pos = (x as i32, y as i32);
                if !occupied.contains(&pos) {
                    spaces.push(pos);
                }
            }
        }
        spaces
    }
}

fn parse_field(input: &str) -> Field {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let blizzards = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '<' => Some(Blizzard {
                    pos: (x as i32, y as i32),
                    dir: (-1, 0),
                }),
                '>' => Some(Blizzard {
                    pos: (x as i32, y as i32),
                    dir: (1, 0),
                }),
                '^' => Some(Blizzard {
                    pos: (x as i32, y as i32),
                    dir: (0, -1),
                }),
                'v' => Some(Blizzard {
                    pos: (x as i32, y as i32),
                    dir: (0, 1),
                }),
                _ => None,
            })
        })
        .collect::<Vec<_>>();

    Field::new(blizzards, width, height)
}

pub fn puzzle_1(input: &str) -> String {
    let mut field = parse_field(input);
    let start = (1, 0);
    let goal = field.get_goal();
    let mut possible_positions = HashSet::from([start.clone()]);

    let mut round = 0;
    loop {
        field.move_blizzards();
        round += 1;

        let free_spaces = field.free_spaces();

        possible_positions = possible_positions
            .into_iter()
            .map(|p| {
                field
                    .neighbors(&p)
                    .into_iter()
                    .filter(|n| free_spaces.contains(n))
            })
            .flatten()
            .collect::<HashSet<_>>();

        let target = possible_positions.iter().find(|&p| goal.eq(p));

        if let Some(_) = target {
            break;
        }
    }

    round.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut field = parse_field(input);
    let start = (1, 0);
    let goal = field.get_goal();
    let mut round = 0;

    // from start to goal
    let mut possible_positions = HashSet::from([start.clone()]);
    loop {
        field.move_blizzards();
        round += 1;

        let free_spaces = field.free_spaces();

        possible_positions = possible_positions
            .into_iter()
            .map(|p| {
                field
                    .neighbors(&p)
                    .into_iter()
                    .filter(|n| free_spaces.contains(n))
            })
            .flatten()
            .collect::<HashSet<_>>();

        let target = possible_positions.iter().find(|&p| goal.eq(p));

        if let Some(_) = target {
            break;
        }
    }

    // back to start
    possible_positions = HashSet::from([goal.clone()]);
    loop {
        field.move_blizzards();
        round += 1;

        let free_spaces = field.free_spaces();

        possible_positions = possible_positions
            .into_iter()
            .map(|p| {
                field
                    .neighbors(&p)
                    .into_iter()
                    .filter(|n| free_spaces.contains(n))
            })
            .flatten()
            .collect::<HashSet<_>>();

        let target = possible_positions.iter().find(|&p| start.eq(p));

        if let Some(_) = target {
            break;
        }
    }

    // to goal again
    possible_positions = HashSet::from([start.clone()]);
    loop {
        field.move_blizzards();
        round += 1;

        let free_spaces = field.free_spaces();

        possible_positions = possible_positions
            .into_iter()
            .map(|p| {
                field
                    .neighbors(&p)
                    .into_iter()
                    .filter(|n| free_spaces.contains(n))
            })
            .flatten()
            .collect::<HashSet<_>>();

        let target = possible_positions.iter().find(|&p| goal.eq(p));

        if let Some(_) = target {
            break;
        }
    }

    round.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "18");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "54");
    }
}
