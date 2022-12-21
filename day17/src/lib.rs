use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
enum Jet {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum Stone {
    Horizontal,
    Plus,
    Corner,
    Vertical,
    Square,
}

impl Stone {
    pub fn lines(&self, x: usize) -> Vec<u8> {
        match self {
            Stone::Horizontal => vec![0b1111000 >> x],
            Stone::Plus => vec![0b0100000 >> x, 0b1110000 >> x, 0b0100000 >> x],
            Stone::Corner => vec![0b1110000 >> x, 0b0010000 >> x, 0b0010000 >> x],
            Stone::Vertical => vec![
                0b1000000 >> x,
                0b1000000 >> x,
                0b1000000 >> x,
                0b1000000 >> x,
            ],
            Stone::Square => vec![0b1100000 >> x, 0b1100000 >> x],
        }
    }

    pub fn width(&self) -> usize {
        match self {
            Stone::Horizontal => 4,
            Stone::Plus => 3,
            Stone::Corner => 3,
            Stone::Vertical => 1,
            Stone::Square => 2,
        }
    }
}

fn parse_jets(input: &str) -> Vec<Jet> {
    input
        .chars()
        .map(|v| {
            if v == '<' {
                return Jet::Left;
            }
            Jet::Right
        })
        .collect::<Vec<_>>()
}

struct Tower {
    lines: Vec<u8>,
    removed_lines: usize,
}

impl Tower {
    pub fn new() -> Self {
        Tower {
            lines: Vec::new(),
            removed_lines: 0,
        }
    }

    pub fn get_height(&self) -> usize {
        self.lines.len()
    }

    pub fn get_total_height(&self) -> usize {
        self.lines.len() + self.removed_lines
    }

    pub fn check_collision(&self, stone: Stone, pos: (usize, usize)) -> bool {
        let h = self.lines.len();
        let y = pos.1;

        if h <= y {
            return false;
        }

        let col = stone.lines(pos.0);

        (0..col.len()).any(|i| y + i < h && (col[i] & self.lines[y + i]) != 0)
    }

    pub fn add_stone(&mut self, stone: Stone, pos: (usize, usize)) {
        let h = self.lines.len();
        let y = pos.1;

        let col = stone.lines(pos.0);

        for i in 0..col.len() {
            if y + i < h {
                self.lines[y + i] |= col[i];
            } else {
                self.lines.push(col[i]);
            }
        }

        if self.lines.len() > 8000 {
            self.lines.drain(0..4000);
            self.removed_lines += 4000;
        }
    }
}

impl Display for Tower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.lines.len()).rev() {
            let line = (0..7)
                .map(|b| {
                    if (0b1000000 >> b) & self.lines[i] != 0 {
                        return '#';
                    }
                    '.'
                })
                .collect::<String>();

            writeln!(f, "{}", line)?
        }
        Ok(())
    }
}

pub fn puzzle_1(input: &str) -> String {
    let jets = parse_jets(input);
    let mut jets = jets.iter().cycle();

    let stones = [
        Stone::Horizontal,
        Stone::Plus,
        Stone::Corner,
        Stone::Vertical,
        Stone::Square,
    ]
    .into_iter()
    .cycle()
    .take(2022);

    let mut tower = Tower::new();

    for stone in stones {
        let mut y = tower.get_height() + 3;
        let mut x = 2;

        loop {
            // cyclic iterator so unwrap is ok
            let jet = jets.next().unwrap();

            // jet move
            match jet {
                Jet::Left => {
                    if x > 0 && !tower.check_collision(stone, (x - 1, y)) {
                        x = x - 1;
                    }
                }
                Jet::Right => {
                    let w = stone.width();
                    if x + w <= 6 && !tower.check_collision(stone, (x + 1, y)) {
                        x = x + 1;
                    }
                }
            }

            // down move
            if y > 0 && !tower.check_collision(stone, (x, y - 1)) {
                y = y - 1;
            } else {
                tower.add_stone(stone, (x, y));
                break;
            }
        }
    }

    let result = tower.get_total_height();

    result.to_string()
}

const STONE_COUNT: usize = 1000000000000;

pub fn puzzle_2(input: &str) -> String {
    let jets = parse_jets(input);
    let jet_count = jets.len();
    let mut jets = jets.iter().cycle();

    let stones = [
        Stone::Horizontal,
        Stone::Plus,
        Stone::Corner,
        Stone::Vertical,
        Stone::Square,
    ]
    .into_iter()
    .cycle()
    .take(jet_count * 20);

    let mut tower = Tower::new();

    let mut heights = Vec::new();

    for stone in stones {
        let mut y = tower.get_height() + 3;
        let mut x = 2;

        loop {
            // cyclic iterator so unwrap is ok
            let jet = jets.next().unwrap();

            // jet move
            match jet {
                Jet::Left => {
                    if x > 0 && !tower.check_collision(stone, (x - 1, y)) {
                        x = x - 1;
                    }
                }
                Jet::Right => {
                    let w = stone.width();
                    if x + w <= 6 && !tower.check_collision(stone, (x + 1, y)) {
                        x = x + 1;
                    }
                }
            }

            // down move
            if y > 0 && !tower.check_collision(stone, (x, y - 1)) {
                y = y - 1;
            } else {
                tower.add_stone(stone, (x, y));
                break;
            }
        }

        heights.push(tower.get_total_height());
    }

    let diffs = heights.windows(2).map(|p| p[1] - p[0]).collect::<Vec<_>>();

    // look for a long repeating sequence of stones which add the same amount of height
    let mut pattern_start = 0;
    let mut pattern_length = 0;
    'outer: for chunk_size in (6..diffs.len() / 2).rev() {
        for offset in 0..diffs.len() - 2 * chunk_size {
            let mut chunks = diffs[offset..].chunks_exact(chunk_size);

            let a = chunks.next().unwrap();
            let b = chunks.next().unwrap();

            if a == b {
                println!(
                    "found repeating part at offset = {} with length: {}",
                    offset, chunk_size
                );
                pattern_start = offset;
                pattern_length = chunk_size;
                break 'outer;
            }
        }
    }

    let pattern_height = heights[pattern_start + pattern_length] - heights[pattern_start];
    let stone_count = STONE_COUNT - pattern_start - 1;
    let modulo = stone_count % pattern_length;
    let result = heights[pattern_start + modulo] + pattern_height * (stone_count / pattern_length);

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "3068");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "1514285714288");
    }
}
