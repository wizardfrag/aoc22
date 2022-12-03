use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            let first: HashSet<char> = first.chars().collect();
            let second: HashSet<char> = second.chars().collect();

            let intersection: Vec<char> = (&first & &second).into_iter().collect();
            intersection.first().unwrap().clone()
        })
        .fold(0, score);

    result.to_string()
}

pub fn part2(input: &str) -> String {
    let result = &input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let rucksacks: Vec<HashSet<char>> = chunk
                .map(|rucksack| rucksack.chars().collect::<HashSet<char>>())
                .collect();
            let intersection1 = &rucksacks[0] & &rucksacks[1];
            let intersection2: Vec<char> = (&intersection1 & &rucksacks[2]).into_iter().collect();
            (intersection2.first().unwrap()).clone()
        })
        .fold(0, score);

    result.to_string()
}

fn score(acc: u32, duplicate: char) -> u32 {
    acc + if ('A'..='Z').contains(&duplicate) {
        duplicate as u32 - 'A' as u32 + 1 + 26
    } else {
        duplicate as u32 - 'a' as u32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "70");
    }
}
