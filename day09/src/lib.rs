use std::collections::HashSet;

#[derive(Clone)]
struct Pos(i32, i32);

impl Pos {
    pub fn follow(&mut self, head: &Pos) {
        let x = self.0 - head.0;
        let y = self.1 - head.1;

        let distance = x.abs() + y.abs();

        if distance >= 3 {
            // move both directions by 1
            self.0 -= x.signum();
            self.1 -= y.signum();
        } else {
            // move the "2" distance direction by 1
            // and the "0" and "1" distance by 0
            self.0 -= x / 2;
            self.1 -= y / 2;
        }
    }
}

#[derive(Copy, Clone)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves = vec![];

    for line in input.lines() {
        let (m, amount) = line.split_once(' ').unwrap();

        let m = match m {
            "R" => Move::Right,
            "L" => Move::Left,
            "U" => Move::Up,
            "D" => Move::Down,
            _ => panic!("unknown move character!"),
        };

        let amount = amount.parse::<usize>().unwrap();

        for _ in 0..amount {
            moves.push(m);
        }
    }

    moves
}

pub fn puzzle_1(input: &str) -> String {
    let moves = parse_moves(input);

    let mut head = Pos(0, 0);
    let mut tail = Pos(0, 0);

    let mut positions = HashSet::from([(tail.0, tail.1)]);

    for m in moves {
        match m {
            Move::Up => head.1 += 1,
            Move::Right => head.0 += 1,
            Move::Down => head.1 -= 1,
            Move::Left => head.0 -= 1,
        }

        tail.follow(&head);
        positions.insert((tail.0, tail.1));
    }

    let count = positions.len();

    count.to_string()
}

const KNOTS: usize = 10;

pub fn puzzle_2(input: &str) -> String {
    let moves = parse_moves(input);

    let mut knots = Vec::from_iter(std::iter::repeat(Pos(0, 0)).take(KNOTS));

    let mut positions = HashSet::from([(knots[0].0, knots[0].0)]);

    for m in moves {
        for i in 0..KNOTS {
            if i == 0 {
                let head = knots.get_mut(i).unwrap();
                match m {
                    Move::Up => head.1 += 1,
                    Move::Right => head.0 += 1,
                    Move::Down => head.1 -= 1,
                    Move::Left => head.0 -= 1,
                }
            } else {
                let front = knots[i - 1].clone();
                let tail = knots.get_mut(i).unwrap();
                tail.follow(&front);
            }
        }

        positions.insert((knots[KNOTS - 1].0, knots[KNOTS - 1].1));
    }

    let count = positions.len();

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "13");
    }

    const INPUT2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "1");

        let result = puzzle_2(INPUT2);
        assert_eq!(result, "36");
    }
}
