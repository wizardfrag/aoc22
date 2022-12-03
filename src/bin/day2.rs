use aoc22::day2::*;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input-day2.txt").unwrap();
    println!("{}", part1(&file));
    println!("{}", part2(&file));
}
