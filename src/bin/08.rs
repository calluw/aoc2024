use std::collections::HashSet;

use glam::IVec2;
use itertools::iproduct;
use nom::character::complete::{char, satisfy};
use nom::Parser;
use nom::{
    branch::alt,
    multi::{many1, many_till},
    IResult,
};
use nom_locate::{position, LocatedSpan};

advent_of_code::solution!(8);

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
struct Antenna<'a> {
    position: Span<'a>,
    letter: char,
}

fn parse_antenna(s: Span) -> IResult<Span, Antenna> {
    let (s, position) = position(s)?;
    let (s, letter) = satisfy(|ch| ch.is_alphanumeric())(s)?;
    Ok((s, Antenna { position, letter }))
}

fn parse_antennae(s: Span) -> IResult<Span, Vec<Antenna>> {
    many1(many_till(alt((char('.'), char('\n'))), parse_antenna).map(|(_, antenna)| antenna))(s)
}

fn in_grid(pos: &IVec2, max_pos: &IVec2) -> bool {
    pos.x >= 0 && pos.x < max_pos.x && pos.y >= 0 && pos.y < max_pos.y
}

fn get_antinodes_for_pair(
    pair: (&Antenna, &Antenna),
    max_pos: IVec2,
    many: bool,
) -> HashSet<IVec2> {
    let (a, b) = pair;

    if a.letter != b.letter {
        // Not a pair of the same antenna type, no antinodes
        return HashSet::new();
    }

    let pos_a = IVec2::new(
        a.position.location_line() as i32 - 1,
        a.position.get_column() as i32 - 1,
    );
    let pos_b = IVec2::new(
        b.position.location_line() as i32 - 1,
        b.position.get_column() as i32 - 1,
    );

    if pos_a == pos_b {
        // Not a pair at all, just the same antenna twice
        return HashSet::new();
    }

    let delta = pos_b - pos_a;

    if !many {
        let antinodes: Vec<IVec2> = vec![pos_b + delta, pos_a - delta];
        let antinodes: Vec<IVec2> = antinodes
            .into_iter()
            .filter(|p| in_grid(p, &max_pos))
            .collect();

        HashSet::from_iter(antinodes)
    } else {
        // NOTE: Bizarrely, antennae are always also antinodes now?
        let mut antinodes = HashSet::new();
        for i in 0.. {
            let antinode = pos_b + i * delta;
            if !in_grid(&antinode, &max_pos) {
                break;
            }
            antinodes.insert(antinode);
        }
        for i in 0.. {
            let antinode = pos_a - i * delta;
            if !in_grid(&antinode, &max_pos) {
                break;
            }
            antinodes.insert(antinode);
        }
        antinodes
    }
}

fn get_unique_antinodes(antennae: &Vec<Antenna>, max_pos: IVec2, many: bool) -> HashSet<IVec2> {
    iproduct!(antennae, antennae)
        .map(|p| get_antinodes_for_pair(p, max_pos, many))
        .reduce(|acc, s| acc.union(&s).cloned().collect())
        .unwrap()
}

fn do_parts(input: &str, many: bool) -> Option<u32> {
    let antennae = parse_antennae(Span::new(input)).unwrap().1;

    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let antinodes = get_unique_antinodes(&antennae, IVec2::new(height, width), many);
    Some(antinodes.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    do_parts(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    do_parts(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
