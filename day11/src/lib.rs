#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    // tuple (a, b, c) representing ax^2 + bx + c
    operation: (u64, u64, u64),
    test_division: u64,
    // tuple for throw targets (true, false)
    targets: (usize, usize),
    inspections: u64,
}

impl Monkey {
    pub fn new(input: &str) -> Self {
        let mut lines = input.lines().skip(1);

        // starting items
        let line = lines.next().unwrap();
        let starting_items = line
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        // operation
        let mut operation = (0, 0, 0);
        let line = lines.next().unwrap();
        let line = line.strip_prefix("  Operation: new = old ").unwrap();
        if line == "* old" {
            operation.0 = 1;
        } else if line.starts_with("* ") {
            let b = line.strip_prefix("* ").unwrap().parse::<u64>().unwrap();
            operation.1 = b;
        } else if line.starts_with("+ ") {
            let c = line.strip_prefix("+ ").unwrap().parse::<u64>().unwrap();
            operation.2 = c;
            operation.1 = 1;
        } else {
            panic!("unknown monkey operation {}", line);
        }

        // test division
        let line = lines.next().unwrap();
        let test_division = line
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<u64>()
            .unwrap();

        //targets
        let line = lines.next().unwrap();
        let t = line
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let line = lines.next().unwrap();
        let f = line
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Monkey {
            items: starting_items,
            operation,
            test_division,
            targets: (t, f),
            inspections: 0,
        }
    }
}

pub fn puzzle_1(input: &str) -> String {
    let mut monkeys = input
        .split("\n\n")
        .map(|m| Monkey::new(m))
        .collect::<Vec<Monkey>>();

    for _round in 0..20 {
        for m in 0..monkeys.len() {
            let monkey = monkeys.get_mut(m).unwrap();

            // list for modified items and their targets
            let mut items = Vec::new();

            let item_count = monkey.items.len();
            for _ in 0..item_count {
                // take the item
                let mut item = monkey.items.remove(0);

                // do the operation
                item = monkey.operation.0 * item * item
                    + monkey.operation.1 * item
                    + monkey.operation.2;

                // monkey gets bored
                item /= 3;

                // test
                let target = if item % monkey.test_division == 0 {
                    monkey.targets.0
                } else {
                    monkey.targets.1
                };

                // move the item
                items.push((target, item));

                // an item was inspected
                monkey.inspections += 1;
            }

            // move the items
            for (t, i) in items {
                let monkey = monkeys.get_mut(t).unwrap();
                monkey.items.push(i);
            }
        }
    }

    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u64>>();
    inspections.sort_by(|a, b| b.cmp(a));

    let business = inspections.iter().take(2).product::<u64>();

    business.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut monkeys = input
        .split("\n\n")
        .map(|m| Monkey::new(m))
        .collect::<Vec<Monkey>>();

    let modulo = monkeys.iter().map(|m| m.test_division).product::<u64>();

    for _round in 0..10000 {
        for m in 0..monkeys.len() {
            let monkey = monkeys.get_mut(m).unwrap();

            // list for modified items and their targets
            let mut items = Vec::new();

            let item_count = monkey.items.len();
            for _ in 0..item_count {
                // take the item
                let mut item = monkey.items.remove(0);

                // do the operation
                item = monkey.operation.0 * item * item
                    + monkey.operation.1 * item
                    + monkey.operation.2;

                // keep the numbers managable
                item %= modulo;

                // test
                let target = if item % monkey.test_division == 0 {
                    monkey.targets.0
                } else {
                    monkey.targets.1
                };

                // move the item
                items.push((target, item));

                // an item was inspected
                monkey.inspections += 1;
            }

            // move the items
            for (t, i) in items {
                let monkey = monkeys.get_mut(t).unwrap();
                monkey.items.push(i);
            }
        }
    }

    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u64>>();
    inspections.sort_by(|a, b| b.cmp(a));

    let business = inspections.iter().take(2).product::<u64>();

    business.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "10605");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "2713310158");
    }
}
