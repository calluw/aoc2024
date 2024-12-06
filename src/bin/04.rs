use glam::IVec2;
use itertools::iproduct;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::multi::separated_list0;
use nom::{multi::many0, IResult};
use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
struct Lookup {
    index: IVec2,
    value: Letter,
}

type LookupChain = Vec<Lookup>;

fn parse_line(input: &str) -> IResult<&str, Vec<Letter>> {
    many0(alt((
        value(Letter::X, char('X')),
        value(Letter::M, char('M')),
        value(Letter::A, char('A')),
        value(Letter::S, char('S')),
    )))(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<Letter>>> {
    separated_list0(char('\n'), parse_line)(input)
}

fn get_p1_lookup_chain(index: IVec2, direction: IVec2) -> LookupChain {
    (1..=4)
        .zip([Letter::M, Letter::A, Letter::S])
        .map(|(i, letter)| Lookup {
            value: letter,
            index: index + (direction * i),
        })
        .collect()
}

fn get_p1_lookup_chains(index: IVec2) -> Vec<LookupChain> {
    iproduct!(-1..=1, -1..=1)
        .map(IVec2::from)
        .map(|direction| get_p1_lookup_chain(index, direction))
        .collect()
}

fn get_p2_lookup_chains(index: IVec2) -> Vec<LookupChain> {
    // Possibilities are all rotation of each other:
    //
    // M M S M S S M S
    //  A   A   A   A
    // S S S M M M M S
    let corners = [
        IVec2::new(-1, -1),
        IVec2::new(1, -1),
        IVec2::new(1, 1),
        IVec2::new(-1, 1),
    ];
    let mut letters = [Letter::M, Letter::M, Letter::S, Letter::S];

    let mut chains = Vec::new();
    for _ in 0..letters.len() {
        chains.push(
            letters
                .iter()
                .zip(&corners)
                .map(|(letter, corner)| Lookup {
                    value: letter.clone(),
                    index: index + corner,
                })
                .collect(),
        );
        // Rotating the letters slice whilst keeping the corners fixed performs a right rotation of the XMAS
        letters.rotate_right(1);
    }

    chains
}

fn chain_matches(chain: &LookupChain, grid: &HashMap<IVec2, Letter>) -> bool {
    for lookup in chain {
        if grid
            .get(&lookup.index)
            .map_or(true, |letter| *letter != lookup.value)
        {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let wordsearch = parse_lines(input).expect("puzzle input to parse").1;
    let mut grid_lookup: HashMap<IVec2, Letter> = HashMap::new();

    for (i, line) in wordsearch.iter().enumerate() {
        for (j, letter) in line.into_iter().enumerate() {
            grid_lookup.insert(IVec2::new(i as i32, j as i32), letter.clone());
        }
    }

    let mut matches = 0;
    for (i, line) in wordsearch.into_iter().enumerate() {
        for (j, letter) in line.into_iter().enumerate() {
            if let Letter::X = letter {
                let lookup_chains = get_p1_lookup_chains(IVec2::new(i as i32, j as i32));
                for lookup_chain in lookup_chains {
                    if chain_matches(&lookup_chain, &grid_lookup) {
                        matches += 1;
                    }
                }
            }
        }
    }

    Some(matches)
}

pub fn part_two(input: &str) -> Option<u32> {
    let wordsearch = parse_lines(input).expect("puzzle input to parse").1;
    let mut grid_lookup: HashMap<IVec2, Letter> = HashMap::new();

    for (i, line) in wordsearch.iter().enumerate() {
        for (j, letter) in line.into_iter().enumerate() {
            grid_lookup.insert(IVec2::new(i as i32, j as i32), letter.clone());
        }
    }

    let mut matches = 0;
    for (i, line) in wordsearch.into_iter().enumerate() {
        for (j, letter) in line.into_iter().enumerate() {
            if let Letter::A = letter {
                let lookup_chains = get_p2_lookup_chains(IVec2::new(i as i32, j as i32));
                for lookup_chain in lookup_chains {
                    if chain_matches(&lookup_chain, &grid_lookup) {
                        matches += 1;
                    }
                }
            }
        }
    }

    Some(matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
