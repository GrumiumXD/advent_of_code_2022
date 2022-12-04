use itertools::Itertools;
struct CleaningSection(u32, u32);

impl CleaningSection {
    fn contains(&self, other: &Self) -> bool {
        if self.0 <= other.0 && self.1 >= other.1 {
            return true;
        }

        false
    }

    fn overlap(&self, other: &Self) -> bool {
        if (self.0 <= other.0 && self.1 >= other.0) || (other.0 <= self.0 && other.1 >= self.0) {
            return true;
        }

        false
    }
}

fn parse_elf(input: &str) -> CleaningSection {
    let section: Vec<u32> = input
        .split('-')
        .map(|id| id.parse::<u32>().unwrap())
        .collect();

    CleaningSection(section[0], section[1])
}

pub fn puzzle_1(input: &str) -> String {
    let pairs: Vec<(CleaningSection, CleaningSection)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|elf| parse_elf(elf))
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let containments = pairs
        .iter()
        .map(|pair| {
            if pair.0.contains(&pair.1) || pair.1.contains(&pair.0) {
                return 1;
            }
            0
        })
        .sum::<u32>();

    containments.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let pairs: Vec<(CleaningSection, CleaningSection)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|elf| parse_elf(elf))
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let overlaps = pairs
        .iter()
        .map(|pair| {
            if pair.0.overlap(&pair.1) {
                return 1;
            }
            0
        })
        .sum::<u32>();

    overlaps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "4");
    }
}
