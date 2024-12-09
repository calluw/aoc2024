use std::{collections::VecDeque, iter};

use nom::{character::complete::satisfy, multi::many0, IResult, Parser};

advent_of_code::solution!(9);

#[derive(Debug, Clone)]
enum Block {
    Free(usize),
    File(usize),
}

fn parse_free(input: &str) -> IResult<&str, Block> {
    satisfy(|ch| ch.is_digit(10))
        .map(|ch| ch.to_string().parse().unwrap())
        .map(Block::Free)
        .parse(input)
}

fn parse_file(input: &str) -> IResult<&str, Block> {
    satisfy(|ch| ch.is_digit(10))
        .map(|ch| ch.to_string().parse().unwrap())
        .map(Block::File)
        .parse(input)
}

fn parse_blocks(input: &str) -> IResult<&str, Vec<Block>> {
    let (input, block_pairs) = many0(parse_file.and(parse_free))(input)?;
    let (input, final_file) = parse_file(input)?;
    Ok((
        input,
        block_pairs
            .into_iter()
            .flat_map(|(file, free)| [file, free])
            .chain(iter::once(final_file))
            .collect(),
    ))
}

#[derive(Debug, Clone)]
enum Space {
    File(u32),
    Empty,
}

pub fn part_one(input: &str) -> Option<u32> {
    let blocks = parse_blocks(input).unwrap().1;
    let mut disk = VecDeque::new();

    let mut file_id = 0;
    for block in blocks {
        disk.extend(match block {
            Block::Free(s) => vec![Space::Empty; s],
            Block::File(s) => {
                let block_spaces = vec![Space::File(file_id); s];
                file_id += 1;
                block_spaces
            }
        });
    }

    //let file_only_spaces: VecDeque<Space> = disk
    //    .iter()
    //    .filter(|s| matches!(s, Space::File(_)))
    //    .cloned()
    //    .collect();

    let empty_num: usize = disk.iter().filter(|s| matches!(s, Space::Empty)).count();
    let disk_size = disk.len();

    let mut compressed_disk = Vec::new();

    let mut empty_filled = 0;
    for space in disk {
        match space {
            Space::File(id) => compressed_disk.push(id),
            Space::Empty => {}
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
