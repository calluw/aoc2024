use nom::Parser;
use std::collections::VecDeque;

use nom::{
    bytes::complete::tag,
    character::complete::{self, char},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    answer: u64,
    nums: VecDeque<u64>,
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    separated_pair(
        complete::u64,
        tag(": "),
        separated_list1(char(' '), complete::u64),
    )
    .map(|(answer, nums)| Equation {
        answer,
        nums: nums.into(),
    })
    .parse(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(char('\n'), parse_equation)(input)
}

fn can_make_answer(answer: u64, mut nums: VecDeque<u64>, allow_concat: bool) -> bool {
    assert!(!nums.is_empty());
    let next_num = nums.pop_back().unwrap();

    if nums.is_empty() {
        // Can only make the answer if the next num is equal to answer
        return next_num == answer;
    }

    if answer > next_num {
        let subanswer_if_add = answer - next_num;
        if can_make_answer(subanswer_if_add, nums.clone(), allow_concat) {
            return true;
        }
    }

    if allow_concat {
        // Take another num, concat them together, then try the same answer again with the stack of
        // those two replaced with the new concatenated number
        let mut new_nums = nums.clone();
        let second_next_num = new_nums.pop_back().unwrap();
        let concatenated_next_num = format!("{}{}", second_next_num, next_num).parse::<u64>();
        // It can get too large to parse, in which case just skip
        if let Ok(concatenated_next_num) = concatenated_next_num {
            new_nums.push_back(concatenated_next_num);
            dbg!(answer, &new_nums);
            if can_make_answer(answer, new_nums, allow_concat) {
                return true;
            }
        }
    }

    // If the subanswer isn't a whole number, there's no way to make it by multiplying...
    if (answer % next_num) == 0 {
        let subanswer_if_mult = answer / next_num;
        if can_make_answer(subanswer_if_mult, nums.clone(), allow_concat) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input).unwrap().1;

    Some(
        equations
            .into_iter()
            .map(|e| (e.answer, can_make_answer(e.answer, e.nums, false)))
            .filter(|(_, works)| *works)
            .map(|(answer, _)| answer)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input).unwrap().1;

    let mut sum = 0;

    for equation in equations {
        let valid = can_make_answer(equation.answer, equation.nums, true);
        if valid {
            dbg!(&equation.answer);
            sum += equation.answer;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
