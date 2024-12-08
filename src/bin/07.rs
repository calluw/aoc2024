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
        // Concatenation doesn't just apply to the next pop_back number, but to the whole remainder
        // of the calculation: so, the concatenation problem just needs to calculate if the
        // remaining nums can create the current answer with the current operand being considered
        // right-trimmed from it
        let mut answer_string = answer.to_string();
        let next_num_string = next_num.to_string();
        if answer_string.len() > next_num_string.len() {
            let split_num = answer_string.split_off(answer_string.len() - next_num_string.len());
            if split_num == next_num_string {
                let subanswer_if_concat = answer_string.parse::<u64>().unwrap();
                if can_make_answer(subanswer_if_concat, nums.clone(), true) {
                    return true;
                }
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

    let mut sum = 0;

    for equation in equations {
        let valid = can_make_answer(equation.answer, equation.nums, false);
        if valid {
            sum += equation.answer;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input).unwrap().1;

    let mut sum = 0;

    for equation in equations {
        let valid = can_make_answer(equation.answer, equation.nums, true);
        if valid {
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
