use std::collections::HashSet;
use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    range: u32,
}

impl Sensor {
    pub fn new(pos: (i32, i32), beacon: (i32, i32)) -> Self {
        let range = pos.0.abs_diff(beacon.0) + pos.1.abs_diff(beacon.1);

        Sensor { pos, beacon, range }
    }

    pub fn cover_at_row(&self, row: i32) -> Range<i32> {
        let vertical_distance = self.pos.1.abs_diff(row);
        if vertical_distance > self.range {
            return 0..0;
        }

        let diff = (self.range as i32) - (vertical_distance as i32);

        (self.pos.0 - diff)..(self.pos.0 + diff + 1)
    }
}

fn position(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)
}

fn parse_sensors(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list0(
        newline,
        separated_pair(
            preceded(tag("Sensor at "), position),
            tag(": "),
            preceded(tag("closest beacon is at "), position),
        )
        .map(|pair| Sensor::new(pair.0, pair.1)),
    )(input)
}

pub fn puzzle_1(input: &str, y: i32) -> String {
    let (_, sensors) = parse_sensors(input).unwrap();

    let mut positions = HashSet::new();

    for s in sensors.iter() {
        let range = s.cover_at_row(y);

        for p in range {
            positions.insert(p);
        }
    }

    // remove the beacons directly on the row from the covered positions
    for s in sensors.iter() {
        if s.beacon.1 == y {
            positions.remove(&s.beacon.0);
        }
    }

    positions.len().to_string()
}

pub fn puzzle_2(input: &str, bounds: (i32, i32)) -> String {
    let (_, sensors) = parse_sensors(input).unwrap();

    let mut target = (-1, -1);

    for row in bounds.0..bounds.1 {
        let mut ranges = sensors
            .iter()
            .filter_map(|s| {
                let range = s.cover_at_row(row);
                if range.is_empty() {
                    return None;
                }
                Some(range)
            })
            .collect::<Vec<Range<i32>>>();

        ranges.sort_by(|a, b| a.start.cmp(&b.start));

        // build a continous range or find a hole
        let start = ranges[0].clone();
        let (complete, hole) = ranges.iter().fold((start, None), |acc, r| {
            if let Some(_) = acc.1 {
                return (0..0, acc.1);
            }

            if acc.0.end < r.start {
                return (0..0, Some(acc.0.end));
            }

            let start = acc.0.start;
            let end = acc.0.end.max(r.end);

            (start..end, None)
        });

        if let Some(x) = hole {
            target = (x, row);
            break;
        } else if complete.start > bounds.0 {
            target = (0, row);
            break;
        } else if complete.end <= bounds.1 {
            target = (bounds.1, row);
            break;
        }
    }
    // println!("target position: ({}, {})", target.0, target.1);

    let frequency = (target.0 as u64) * 4000000 + target.1 as u64;

    frequency.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT, 10);
        assert_eq!(result, "26");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT, (0, 20));
        assert_eq!(result, "56000011");
    }
}
