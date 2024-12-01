use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res, IResult};
use std::{collections::BTreeMap, iter::zip, str::FromStr};

advent_of_code::solution!(1);

struct Pair(u32, u32);

impl Pair {
    fn from_tuple(t: (u32, u32)) -> Self {
        Self(t.0, t.1)
    }

    fn parse_from_line(input: &str) -> IResult<&str, Self> {
        let (input, first) = map_res(digit1, u32::from_str)(input)?;
        let (input, _) = tag("   ")(input)?;
        let (input, second) = map_res(digit1, u32::from_str)(input)?;
        Ok((input, Self(first, second)))
    }

    fn distance(&self) -> u32 {
        self.1.abs_diff(self.0)
    }
}

fn get_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(Pair::parse_from_line)
        .map(|r| r.expect("parsing puzzle input to succeed"))
        .map(|(_, p)| (p.0, p.1))
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first_list, mut second_list): (Vec<u32>, Vec<u32>) = get_lists(input);

    first_list.sort();
    second_list.sort();

    Some(
        zip(first_list, second_list)
            .map(Pair::from_tuple)
            .map(|p| p.distance())
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first_list, second_list): (Vec<u32>, Vec<u32>) = get_lists(input);

    let mut counts = BTreeMap::<u32, u32>::new();

    for num in second_list {
        *counts.entry(num).or_insert(0) += 1;
    }

    Some(
        first_list
            .iter()
            .map(|n| n * counts.get(n).unwrap_or(&0))
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
