#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

fn score(opp: Move, myself: Move) -> u32 {
    match (opp, myself) {
        (Move::Rock, Move::Rock) => 3 + 1,
        (Move::Rock, Move::Paper) => 6 + 2,
        (Move::Rock, Move::Scissor) => 0 + 3,
        (Move::Paper, Move::Rock) => 0 + 1,
        (Move::Paper, Move::Paper) => 3 + 2,
        (Move::Paper, Move::Scissor) => 6 + 3,
        (Move::Scissor, Move::Rock) => 6 + 1,
        (Move::Scissor, Move::Paper) => 0 + 2,
        (Move::Scissor, Move::Scissor) => 3 + 3,
    }
}

pub fn puzzle_1(input: &str) -> String {
    let rounds: Vec<(Move, Move)> = input
        .lines()
        .map(|moves| {
            let mut encrypted = moves.split_whitespace();
            // parse opponent
            let opp = match encrypted.next() {
                Some("A") => Move::Rock,
                Some("B") => Move::Paper,
                Some("C") => Move::Scissor,
                _ => panic!("unknown move from opponent"),
            };
            // parse my move
            let myself = match encrypted.next() {
                Some("X") => Move::Rock,
                Some("Y") => Move::Paper,
                Some("Z") => Move::Scissor,
                _ => panic!("unknown move for myself"),
            };
            // return the move tuple
            (opp, myself)
        })
        .collect();

    let points: u32 = rounds.iter().map(|r| score(r.0, r.1)).sum();

    points.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let rounds: Vec<(Move, Move)> = input
        .lines()
        .map(|moves| {
            let mut encrypted = moves.split_whitespace();
            // parse opponent
            let opp = match encrypted.next() {
                Some("A") => Move::Rock,
                Some("B") => Move::Paper,
                Some("C") => Move::Scissor,
                _ => panic!("unknown move from opponent"),
            };
            // parse my move
            let myself = match encrypted.next() {
                Some("X") => match opp {
                    Move::Rock => Move::Scissor,
                    Move::Paper => Move::Rock,
                    Move::Scissor => Move::Paper,
                },
                Some("Y") => opp,
                Some("Z") => match opp {
                    Move::Rock => Move::Paper,
                    Move::Paper => Move::Scissor,
                    Move::Scissor => Move::Rock,
                },
                _ => panic!("unknown move for myself"),
            };
            // return the move tuple
            (opp, myself)
        })
        .collect();

    let points: u32 = rounds.iter().map(|r| score(r.0, r.1)).sum();

    points.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
A Y
B X
C Z";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "12");
    }
}
