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
    File(u64),
    Empty,
}

pub fn part_one(input: &str) -> Option<u64> {
    let blocks = parse_blocks(input).unwrap().1;
    let mut disk = Vec::new();

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

    let mut file_only_spaces: VecDeque<u64> = disk
        .iter()
        .filter(|s| matches!(s, Space::File(_)))
        .map(|s| if let Space::File(i) = s { i } else { panic!() })
        .cloned()
        .collect();

    let num_file_spaces = file_only_spaces.len();

    let mut compressed_disk = Vec::new();

    for (i, space) in disk.into_iter().enumerate() {
        if i >= num_file_spaces {
            break;
        }
        match space {
            Space::File(id) => compressed_disk.push(id),
            Space::Empty => compressed_disk.push(file_only_spaces.pop_back().unwrap()),
        }
    }

    assert_eq!(compressed_disk.len(), num_file_spaces);

    Some(
        compressed_disk
            .into_iter()
            .enumerate()
            .map(|(i, num)| i as u64 * num)
            .sum(),
    )
}

#[derive(Debug, Clone)]
enum IdBlock {
    Free(usize),
    File(usize, u64),
}

pub fn part_two(input: &str) -> Option<u64> {
    let blocks = parse_blocks(input).unwrap().1;
    let mut id_blocks = Vec::new();

    // Probably have to operate on whole blocks now
    let mut file_id = 0;
    for block in blocks {
        id_blocks.push(match block {
            Block::Free(s) => IdBlock::Free(s),
            Block::File(s) => {
                let id_block = IdBlock::File(s, file_id);
                file_id += 1;
                id_block
            }
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_short_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 0,
        ));
        assert_eq!(result, Some(60));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
