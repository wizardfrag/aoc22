use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

pub fn part1(input: &str) -> String {
    let (_, assignments) = get_assignments(input).unwrap();

    let result = assignments
        .iter()
        .filter(|(elf1, elf2)| {
            let elf1_is_in_elf2 = elf1.clone().into_iter().all(|num| elf2.contains(&num));
            let elf2_is_in_elf1 = elf2.clone().into_iter().all(|num| elf1.contains(&num));
            elf1_is_in_elf2 || elf2_is_in_elf1
        })
        .count();
    result.to_string()
}

pub fn part2(input: &str) -> String {
    let (_, assignments) = get_assignments(input).unwrap();

    let result = assignments
        .iter()
        .filter(|(elf1, elf2)| elf1.clone().into_iter().any(|num| elf2.contains(&num)))
        .count();
    result.to_string()
}

fn get_elf_sections(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, (start, end)) = separated_pair(complete::u32, tag("-"), complete::u32)(input)?;

    Ok((input, start..=end))
}

fn get_sections(line: &str) -> IResult<&str, Section> {
    let (input, (start, end)) =
        nom::sequence::separated_pair(get_elf_sections, tag(","), get_elf_sections)(line)?;

    Ok((input, (start, end)))
}

type Section = (RangeInclusive<u32>, RangeInclusive<u32>);

fn get_assignments(input: &str) -> IResult<&str, Vec<Section>> {
    let (input, ranges) = separated_list1(newline, get_sections)(input)?;

    Ok((input, ranges))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "4");
    }
}
