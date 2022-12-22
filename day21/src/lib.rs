use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
}

#[derive(Debug, Clone)]
enum Job {
    Value(i64),
    Expression(String, String, Op),
    Wrong,
    Success,
}

fn parse_job(input: &str) -> IResult<&str, Job> {
    alt((
        complete::i64.map(|n| Job::Value(n)),
        tuple((
            alpha1,
            alt((tag(" + "), tag(" - "), tag(" * "), tag(" / "))),
            alpha1,
        ))
        .map(|v: (&str, &str, &str)| {
            let op = match v.1 {
                " + " => Op::Add,
                " - " => Op::Subtract,
                " * " => Op::Multiply,
                _ => Op::Divide,
            };
            Job::Expression(v.0.to_string(), v.2.to_string(), op)
        }),
    ))(input)
}

fn parse_monkeys(input: &str) -> HashMap<String, Job> {
    input
        .lines()
        .map(|l| {
            let (id, job) = l.split_once(": ").unwrap();

            let (_, job) = parse_job(job).unwrap();

            (id.to_string(), job)
        })
        .collect()
}

fn parse_monkeys_2(input: &str) -> HashMap<String, Job> {
    input
        .lines()
        .map(|l| {
            let (id, job) = l.split_once(": ").unwrap();

            let (_, job) = parse_job(job).unwrap();

            if id == "root" {
                if let Job::Expression(a, b, _) = job {
                    return (id.to_string(), Job::Expression(a, b, Op::Equal));
                }
            } else if id == "humn" {
                return (id.to_string(), Job::Wrong);
            }

            (id.to_string(), job)
        })
        .collect()
}

fn resolve_monkeys(monkeys: &mut HashMap<String, Job>) {
    loop {
        let updates = monkeys
            .iter()
            .filter_map(|(id, job)| match job {
                Job::Value(_) => None,
                Job::Expression(a, b, op) => {
                    let a = monkeys[a.as_str()].clone();

                    if let Job::Value(a) = a {
                        let b = monkeys[b.as_str()].clone();
                        if let Job::Value(b) = b {
                            match op {
                                Op::Add => return Some((id.to_string(), Job::Value(a + b))),
                                Op::Subtract => return Some((id.to_string(), Job::Value(a - b))),
                                Op::Multiply => return Some((id.to_string(), Job::Value(a * b))),
                                Op::Divide => return Some((id.to_string(), Job::Value(a / b))),
                                Op::Equal => {
                                    if a == b {
                                        return Some((id.to_string(), Job::Success));
                                    }
                                    return Some((id.to_string(), Job::Wrong));
                                }
                            }
                        }
                    }
                    None
                }
                Job::Wrong => None,
                Job::Success => None,
            })
            .collect::<HashMap<_, _>>();

        if updates.is_empty() {
            break;
        }

        for (id, new_job) in updates.into_iter() {
            let job = monkeys.get_mut(&id).unwrap();
            *job = new_job;
        }
    }
}

pub fn puzzle_1(input: &str) -> String {
    let mut monkeys = parse_monkeys(input);

    resolve_monkeys(&mut monkeys);

    let result = match monkeys["root"] {
        Job::Value(a) => a,
        _ => 0,
    };

    result.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut monkeys = parse_monkeys_2(input);

    // partially solve the jobs
    resolve_monkeys(&mut monkeys);

    // filter out the value monkeys that are not used anymore
    let filtered_monkeys = monkeys
        .iter()
        .filter_map(|m| match m.1 {
            Job::Value(_) => {
                let found = monkeys.iter().find(|m2| match m2.1 {
                    Job::Expression(a, b, _) => a == m.0 || b == m.0,
                    _ => false,
                });
                if let Some(_) = found {
                    return Some((m.0.to_string(), m.1.clone()));
                }
                None
            }
            _ => return Some((m.0.to_string(), m.1.clone())),
        })
        .collect::<HashMap<String, Job>>();

    let mut c_id = "root";
    let mut c_v = 0;

    // go backwards starting from "root"
    loop {
        let j = &filtered_monkeys[c_id];
        if let Job::Expression(a, b, op) = j {
            let (value, other, left) = if let Job::Value(v) = filtered_monkeys[a] {
                (v, b.as_str(), true)
            } else if let Job::Value(v) = filtered_monkeys[b] {
                (v, a.as_str(), false)
            } else {
                panic!("one side should be a value");
            };

            match op {
                Op::Add => {
                    c_v = c_v - value;
                }
                Op::Subtract => {
                    if left {
                        c_v = value - c_v;
                    } else {
                        c_v = c_v + value;
                    }
                }
                Op::Multiply => {
                    c_v = c_v / value;
                }
                Op::Divide => {
                    if left {
                        c_v = value / c_v;
                    } else {
                        c_v = c_v * value;
                    }
                }
                Op::Equal => {
                    c_v = value;
                }
            }
            c_id = other;
        } else if c_id == "humn" {
            break;
        } else {
            panic!("should only be an expression of the humn job");
        }
    }

    c_v.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "152");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "301");
    }
}
