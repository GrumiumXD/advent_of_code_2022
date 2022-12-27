use std::fmt::Display;

struct SNAFUNumber {
    digits: Vec<char>,
}

impl SNAFUNumber {
    pub fn new(digits: Vec<char>) -> Self {
        let valid = vec!['0', '1', '2', '-', '='];
        for c in digits.iter() {
            if !valid.contains(c) {
                panic!("invalid snafu digit: {}", c);
            }
        }

        SNAFUNumber { digits }
    }

    pub fn decimal(&self) -> i64 {
        self.digits
            .iter()
            .enumerate()
            .map(|(index, c)| {
                let factor = 5i64.pow(index as u32);
                match c {
                    '0' => 0,
                    '1' => factor,
                    '2' => 2 * factor,
                    '-' => -1 * factor,
                    '=' => -2 * factor,
                    _ => panic!("invalid digit for conversion: {}", c),
                }
            })
            .sum()
    }
}

impl Display for SNAFUNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.digits.iter().rev() {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

impl From<i64> for SNAFUNumber {
    fn from(value: i64) -> Self {
        if value == 0 {
            return SNAFUNumber::new(vec!['0']);
        }

        let mut number = value;
        let mut pos = 0;
        let mut digits = Vec::new();

        loop {
            let n = 5i64.pow(pos);
            let current = number / n;
            let remainder = current.rem_euclid(5);
            let remainder = match remainder {
                0..=2 => remainder,
                3 => -2,
                4 => -1,
                _ => panic!("panic invalid remainder {}", remainder),
            };

            digits.push(match remainder {
                0 => '0',
                1 => '1',
                2 => '2',
                -1 => '-',
                -2 => '=',
                _ => 'x',
            });

            number -= n * remainder;
            if number == 0 {
                break;
            }

            pos += 1;
        }

        SNAFUNumber::new(digits)
    }
}

impl From<&str> for SNAFUNumber {
    fn from(value: &str) -> Self {
        SNAFUNumber::new(value.chars().rev().collect::<Vec<_>>())
    }
}

fn parse_input(input: &str) -> Vec<SNAFUNumber> {
    input.lines().map(|l| l.into()).collect()
}

pub fn puzzle_1(input: &str) -> String {
    let numbers = parse_input(input);

    let result = SNAFUNumber::from(numbers.iter().map(|n| n.decimal()).sum::<i64>());

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "2=-1=0");
    }
}
