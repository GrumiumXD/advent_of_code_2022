fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => (item as u32) - 96,
        'A'..='Z' => (item as u32) - 38,
        _ => panic!("unknown item type"),
    }
}

pub fn puzzle_1(input: &str) -> String {
    let rucksacks: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let priorities: Vec<u32> = rucksacks
        .iter()
        .map(|rucksack| {
            let count = rucksack.len();
            for i in 0..count / 2 {
                for j in (count / 2)..count {
                    if rucksack[i] == rucksack[j] {
                        return priority(rucksack[i]);
                    }
                }
            }
            0
        })
        .collect();

    let sum: u32 = priorities.iter().sum();

    sum.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let rucksacks: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let priorities: Vec<u32> = rucksacks
        .chunks(3)
        .map(|group| {
            let a = &group[0];
            let b = &group[1];
            let c = &group[2];

            for item in a {
                if b.contains(item) && c.contains(item) {
                    return priority(*item);
                }
            }

            0
        })
        .collect();

    let sum: u32 = priorities.iter().sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "70");
    }
}
