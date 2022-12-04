use aoc22::day4::*;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input-day4.txt").unwrap();
    println!("{}", part1(&file));
    println!("{}", part2(&file));
}
