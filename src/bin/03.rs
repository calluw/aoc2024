use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar, char},
    multi::{many0, many_till},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

advent_of_code::solution!(3);

enum Instruction {
    Mul((u32, u32)),
    Do,
    Dont,
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    preceded(
        tag("mul"),
        delimited(
            char('('),
            separated_pair(complete::u32, char(','), complete::u32),
            char(')'),
        ),
    )
    .map(Instruction::Mul)
    .parse(input)
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    tag("do()").map(|_| Instruction::Do).parse(input)
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    tag("don't()").map(|_| Instruction::Dont).parse(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_mul, parse_do, parse_dont))(input)
}

fn parse_instruction_from_corrupted(input: &str) -> IResult<&str, Instruction> {
    let (input, (_discard, mul)) = many_till(anychar, parse_instruction)(input)?;
    Ok((input, mul))
}

fn parse_instructions_from_corrupted(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(parse_instruction_from_corrupted)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_instructions_from_corrupted(input)
        .expect("puzzle input to parse")
        .1;

    let mut sum = 0;
    for instruction in instructions {
        if let Instruction::Mul((a, b)) = instruction {
            sum += a * b;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = parse_instructions_from_corrupted(input)
        .expect("puzzle input to parse")
        .1;

    let mut sum = 0;
    let mut enabled = true;
    for instruction in instructions {
        match instruction {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul((a, b)) => {
                if enabled {
                    sum += a * b
                } else {
                }
            }
        };
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
