fn parse_numbers(input: &str) -> Vec<Vec<u32>> {
    let numbers: Vec<Vec<u32>> = input
        .split("\n\n")
        .map(|chunk| chunk.lines().map(|l| l.parse::<u32>().unwrap()).collect())
        .collect();

    numbers
}

pub fn puzzle_1(input: &str) -> String {
    let numbers = parse_numbers(input);

    let max = numbers
        .iter()
        .map(|n| {
            let s: u32 = n.iter().sum();
            s
        })
        .max()
        .unwrap();

    max.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let numbers = parse_numbers(input);

    let mut elves: Vec<u32> = numbers
        .iter()
        .map(|n| {
            let s: u32 = n.iter().sum();
            s
        })
        .collect();

    elves.sort_by(|a, b| b.cmp(a));

    let top_3: u32 = elves.iter().take(3).sum();

    top_3.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "45000");
    }
}
