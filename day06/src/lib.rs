use std::collections::HashSet;

const PACKET_LENGTH: usize = 4;

pub fn puzzle_1(input: &str) -> String {
    let sequence = input.chars().collect::<Vec<char>>();

    let mut index = 0;

    for i in PACKET_LENGTH..sequence.len() {
        // get the last characters from the sequence
        // and collect them in a set
        let set = sequence
            .iter()
            .enumerate()
            .filter_map(|(idx, c)| {
                if idx >= i - PACKET_LENGTH && idx < i {
                    Some(*c)
                } else {
                    None
                }
            })
            .collect::<HashSet<char>>();

        // check the set length to find if there were duplicates
        if set.len() == PACKET_LENGTH {
            index = i;
            break;
        }
    }

    index.to_string()
}

const MESSAGE_LENGTH: usize = 14;

pub fn puzzle_2(input: &str) -> String {
    let sequence = input.chars().collect::<Vec<char>>();

    let mut index = 0;

    for i in MESSAGE_LENGTH..sequence.len() {
        // get the last characters from the sequence
        // and collect them in a set
        let set = sequence
            .iter()
            .enumerate()
            .filter_map(|(idx, c)| {
                if idx >= i - MESSAGE_LENGTH && idx < i {
                    Some(*c)
                } else {
                    None
                }
            })
            .collect::<HashSet<char>>();

        // check the set length to find if there were duplicates
        if set.len() == MESSAGE_LENGTH {
            index = i;
            break;
        }
    }

    index.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "7");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "19");
    }
}
