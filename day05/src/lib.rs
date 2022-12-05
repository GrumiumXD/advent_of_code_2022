fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    // traverse the stack rows from the bottom up
    let mut lines = input.lines().rev();

    // use the bottom row for counting the stacks
    let stack_count = lines.next().unwrap().matches(char::is_numeric).count();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    // traverse the other rows
    for row in lines {
        row.chars().skip(1).step_by(4).enumerate().for_each(|item| {
            if item.1 != ' ' {
                stacks[item.0].push(item.1);
            }
        });
    }

    stacks
}

#[derive(Debug)]
struct Move {
    amount: u32,
    source: usize,
    target: usize,
}

fn parse_moves(input: &str) -> Vec<Move> {
    let moves = input
        .lines()
        .map(|line| {
            let numbers = line
                .strip_prefix("move ")
                .unwrap()
                .replace(" from ", ",")
                .replace(" to ", ",")
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            Move {
                amount: numbers[0] as u32,
                source: numbers[1] - 1,
                target: numbers[2] - 1,
            }
        })
        .collect::<Vec<Move>>();

    moves
}

pub fn puzzle_1(input: &str) -> String {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    let moves = parse_moves(moves);
    let mut stacks = parse_stacks(stacks);

    for m in moves.iter() {
        for _ in 0..m.amount {
            let src = stacks.get_mut(m.source).unwrap().pop().unwrap();
            stacks.get_mut(m.target).unwrap().push(src);
        }
    }

    let tops = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();

    tops
}

pub fn puzzle_2(input: &str) -> String {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    let moves = parse_moves(moves);
    let mut stacks = parse_stacks(stacks);

    for m in moves.iter() {
        // get the soruce stack
        let stack = stacks.get_mut(m.source).unwrap();

        let pos = stack.len() - (m.amount as usize);
        // pull the crates from the stack
        let mut items = stack.drain(pos..).collect::<Vec<char>>();

        // get the target stack
        let stack = stacks.get_mut(m.target).unwrap();

        // put the items on the stack
        stack.append(&mut items);
    }

    let tops = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();

    tops
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "MCD");
    }
}
