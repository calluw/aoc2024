use std::{
    collections::{HashSet, VecDeque},
    iter,
};

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

    // Probably have to operate on whole blocks now, so give them IDs
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

    let file_only_blocks: Vec<IdBlock> = id_blocks
        .iter()
        .filter(|s| matches!(s, IdBlock::File(_, _)))
        .cloned()
        .collect();

    let mut seen_file_ids = HashSet::new();

    let mut compressed_blocks = Vec::new();
    for block in id_blocks {
        match block {
            IdBlock::File(s, id) => {
                if seen_file_ids.contains(&id) {
                    // This was either already moved to the left to fill a
                    // space, so ignore it by pretend its now a space: because
                    // of the left to right scan, its not possible that this
                    // space could ever be used to fill something in so it
                    // doesn't matter
                    compressed_blocks.push(IdBlock::Free(s));
                } else {
                    seen_file_ids.insert(id);
                    compressed_blocks.push(block);
                }
            }
            IdBlock::Free(s) => {
                // Attempt to use the free space by backwards-iterating the
                // available blocks to move in: if we use a block, add it to the
                // seen IDs so that it isn't later counted as a file and added
                // to the compressed disk
                let mut current_space = s;
                for file_block in file_only_blocks.iter().rev() {
                    if let IdBlock::File(file_s, id) = file_block {
                        if seen_file_ids.contains(id) {
                            // This one was also already substituted elsewhere,
                            // can't reuse
                            continue;
                        }
                        if *file_s <= current_space {
                            current_space -= *file_s;
                            compressed_blocks.push(file_block.clone());
                            seen_file_ids.insert(*id);
                        }
                    } else {
                        panic!();
                    }
                }

                if current_space > 0 {
                    compressed_blocks.push(IdBlock::Free(current_space));
                }
            }
        }
    }

    let mut compressed_disk = Vec::new();

    for block in compressed_blocks {
        match block {
            IdBlock::Free(s) => {
                for _ in 0..s {
                    compressed_disk.push(Space::Empty);
                }
            }
            IdBlock::File(s, id) => {
                for _ in 0..s {
                    compressed_disk.push(Space::File(id));
                }
            }
        }
    }

    Some(
        compressed_disk
            .iter()
            .enumerate()
            .map(|(i, space)| match space {
                Space::Empty => 0,
                Space::File(id) => i as u64 * id,
            })
            .sum(),
    )
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
