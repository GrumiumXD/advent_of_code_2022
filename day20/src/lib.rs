#[derive(Eq, PartialEq, Clone, Debug)]
struct Element {
    id: usize,
    value: i64,
}

fn parse_input(input: &str) -> Vec<Element> {
    input
        .split("\n")
        .enumerate()
        .map(|l| Element {
            id: l.0,
            value: l.1.parse::<i64>().unwrap(),
        })
        .collect::<Vec<_>>()
}

pub fn puzzle_1(input: &str) -> String {
    let mut elements = parse_input(input);
    let original_order = elements.clone();
    let rem = original_order.len() as isize - 1;

    for el in original_order.iter() {
        let index = elements
            .iter()
            .position(|s| {
                if el == s {
                    return true;
                }
                false
            })
            .unwrap();

        let current = elements.remove(index);

        let new_pos = (index as isize) + (current.value as isize);
        let new_pos = new_pos.rem_euclid(rem) as usize;

        elements.insert(new_pos, current);
    }

    let mut last_pos = None;
    let result = elements
        .iter()
        .cycle()
        .enumerate()
        .filter_map(|(index, el)| {
            if let Some(pos) = last_pos {
                if index == pos + 1000 {
                    last_pos = Some(index);
                    return Some(el.value);
                }
            } else {
                // search for 0
                if el.value == 0 {
                    last_pos = Some(index);
                }
            }
            None
        })
        .take(3)
        .sum::<i64>();

    result.to_string()
}

const KEY: i64 = 811589153;

pub fn puzzle_2(input: &str) -> String {
    let mut elements = parse_input(input);
    let original_order = elements.clone();
    let rem = original_order.len() as isize - 1;

    for el in elements.iter_mut() {
        el.value *= KEY;
    }

    for _ in 0..10 {
        for el in original_order.iter() {
            let index = elements
                .iter()
                .position(|s| {
                    if el.id == s.id {
                        return true;
                    }
                    false
                })
                .unwrap();

            let current = elements.remove(index);

            let new_pos = (index as isize) + (current.value as isize);
            let new_pos = new_pos.rem_euclid(rem) as usize;

            elements.insert(new_pos, current);
        }
    }

    let mut last_pos = None;
    let result = elements
        .iter()
        .cycle()
        .enumerate()
        .filter_map(|(index, el)| {
            if let Some(pos) = last_pos {
                if index == pos + 1000 {
                    last_pos = Some(index);
                    return Some(el.value);
                }
            } else {
                // search for 0
                if el.value == 0 {
                    last_pos = Some(index);
                }
            }
            None
        })
        .take(3)
        .sum::<i64>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1
2
-3
3
-2
0
4";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "3");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "1623178306");
    }
}
