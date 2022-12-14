use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone)]
enum Value {
    Integer(u32),
    List(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Value {}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer(i1), Value::Integer(i2)) => i1.cmp(i2),
            (Value::Integer(i), Value::List(_)) => {
                let tmp_list = vec![Value::Integer(*i)];
                let entry = Value::List(tmp_list);
                entry.cmp(other)
            }
            (Value::List(_), Value::Integer(_)) => {
                let res = other.cmp(self);
                res.reverse()
            }
            (Value::List(l1), Value::List(l2)) => {
                let length = l1.len().min(l2.len());

                for i in 0..length {
                    let e1 = &l1[i];

                    let e2 = &l2[i];

                    let res = e1.cmp(e2);

                    if res != Ordering::Equal {
                        return res;
                    }
                }

                return l1.len().cmp(&l2.len());
            }
        }
    }
}

fn packet(input: &str) -> IResult<&str, Value> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]"))
            .map(|vec| Value::List(vec)),
        nom::character::complete::u32.map(|num| Value::Integer(num)),
    ))(input)
}

fn pairs(input: &str) -> IResult<&str, Vec<(Value, Value)>> {
    separated_list1(tag("\n\n"), separated_pair(packet, newline, packet))(input)
}

pub fn puzzle_1(input: &str) -> String {
    let (_input, packets) = pairs(input).unwrap();

    let indicies = packets
        .iter()
        .enumerate()
        .filter_map(|(idx, (p1, p2))| {
            if p1 < p2 {
                return Some(idx + 1);
            }
            None
        })
        .collect::<Vec<usize>>();

    let sum = indicies.iter().sum::<usize>();

    sum.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (_input, packets) = pairs(input).unwrap();

    // flatten the pairs
    let mut packets = packets
        .iter()
        .flat_map(|(p1, p2)| vec![p1.clone(), p2.clone()])
        .collect::<Vec<Value>>();

    // add the divider packtes
    let d1 = Value::List(vec![Value::List(vec![Value::Integer(2)])]);
    let d2 = Value::List(vec![Value::List(vec![Value::Integer(6)])]);
    packets.push(d1.clone());
    packets.push(d2.clone());

    packets.sort();

    let i1 = packets.binary_search(&d1).unwrap();
    let i2 = packets.binary_search(&d2).unwrap();

    ((i1 + 1) * (i2 + 1)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "140");
    }
}
