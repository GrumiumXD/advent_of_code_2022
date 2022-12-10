enum Instruction {
    Noop,
    AddX(i32),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let ins = input
        .lines()
        .map(|l| {
            if l == "noop" {
                return Instruction::Noop;
            }

            let amount = l.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
            Instruction::AddX(amount)
        })
        .collect::<Vec<Instruction>>();

    ins
}

pub fn puzzle_1(input: &str) -> String {
    let instructions = parse_instructions(input);
    let mut cycles = Vec::from([1]);

    for ins in instructions {
        let last = cycles.last().unwrap().clone();
        match ins {
            Instruction::Noop => cycles.push(last),
            Instruction::AddX(value) => {
                cycles.push(last);
                cycles.push(last + value);
            }
        }
    }

    // filter the required cycles and calculate their strength
    let filtered_cycles = cycles
        .iter()
        .enumerate()
        .filter_map(|(idx, v)| {
            let cycle_index = idx + 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle_index) {
                return Some(*v * cycle_index as i32);
            }
            None
        })
        .collect::<Vec<i32>>();

    let sum = filtered_cycles.iter().sum::<i32>();

    sum.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let instructions = parse_instructions(input);
    let mut x_values = Vec::from([1]);

    for ins in instructions {
        let last = x_values.last().unwrap().clone();
        match ins {
            Instruction::Noop => x_values.push(last),
            Instruction::AddX(value) => {
                x_values.push(last);
                x_values.push(last + value);
            }
        }
    }

    let mut pixels = Vec::<char>::new();
    for i in 0..240 {
        let x = i % 40;
        let v = x_values[i];

        if (v - x as i32).abs() <= 1 {
            pixels.push('#');
        } else {
            pixels.push('.');
        }
    }

    let lines = pixels
        .chunks(40)
        .map(|s| s.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    lines.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "13140");
    }

    const RENDERED_IMAGE: &str = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, RENDERED_IMAGE);
    }
}
