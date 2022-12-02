// use day01;
use day02;
use std::fs::read_to_string;

fn main() {
    // println!("Day 01");
    // let input = read_to_string("inputs/day01.txt").unwrap();
    // let result = day01::puzzle_1(&input);
    // println!("puzzle 1: {}", result);
    // let result = day01::puzzle_2(&input);
    // println!("puzzle 1: {}", result);

    println!("Day 02");
    let input = read_to_string("inputs/day02.txt").unwrap();
    let result = day02::puzzle_1(&input);
    println!("puzzle 1: {}", result);
    let result = day02::puzzle_2(&input);
    println!("puzzle 1: {}", result);
}
