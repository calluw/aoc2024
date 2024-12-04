use glam::IVec2;
use itertools::iproduct;
use nom::branch::alt;
use nom::character::complete::char;
use nom::multi::separated_list0;
use nom::Parser;
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
        char('X').map(|_| Letter::X),
        char('M').map(|_| Letter::M),
        char('A').map(|_| Letter::A),
        char('S').map(|_| Letter::S),
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

#[rustfmt::skip]
fn get_p2_lookup_chain(index: IVec2) -> LookupChain {
    vec![
        // TODO: Seems that actually, the Ms and Ss can be orientated in any way such that it forms
        // the X-MAS, not just the specific way in the example, need to check them all...
        Lookup { value: Letter::M, index: index + IVec2::new(-1, -1) },
        Lookup { value: Letter::M, index: index + IVec2::new(1, -1) },
        Lookup { value: Letter::S, index: index + IVec2::new(1, 1) },
        Lookup { value: Letter::S, index: index + IVec2::new(-1, 1) },
    ]
}

fn chain_matches(chain: &LookupChain, grid: &HashMap<IVec2, Letter>) -> bool {
    for lookup in chain {
        if grid
            .get(&lookup.index)
            .map_or(true, |letter| *letter != lookup.value)
        {
            println!(
                "Found mismatch: {:?} is not {:?}",
                lookup.index, lookup.value
            );
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
                println!("Looking at A: {:?}", (i, j));
                let lookup_chain = get_p2_lookup_chain(IVec2::new(i as i32, j as i32));
                if chain_matches(&lookup_chain, &grid_lookup) {
                    println!("Matched {:?} with chain: {:?}", (i, j), lookup_chain);
                    matches += 1;
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
