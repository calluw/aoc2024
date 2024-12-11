use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::iproduct;
use nom::{
    character::complete::{char, satisfy},
    multi::{many1, separated_list0},
    IResult, Parser,
};
use nom_locate::{position, LocatedSpan};

advent_of_code::solution!(10);

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
struct Square<'a> {
    position: Span<'a>,
    value: u32,
}

fn parse_square(s: Span) -> IResult<Span, Square> {
    let (s, position) = position(s)?;
    let (s, value) = satisfy(|ch| ch.is_numeric())
        .map(|ch| ch.to_digit(10).unwrap())
        .parse(s)?;
    Ok((s, Square { position, value }))
}

fn parse_map(s: Span) -> IResult<Span, Vec<Vec<Square>>> {
    separated_list0(char('\n'), many1(parse_square))(s)
}

fn get_neighbours(position: &IVec2) -> [IVec2; 4] {
    [
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
        IVec2::new(0, -1),
        IVec2::new(0, 1),
    ]
    .map(|dx| position + dx)
}

fn get_trailends_reachable(position: &IVec2, map_lookup: &HashMap<IVec2, u32>) -> HashSet<IVec2> {
    let position_value = map_lookup.get(position);

    if position_value.is_none() {
        return HashSet::new();
    }

    let position_value = *position_value.unwrap();

    if position_value == 9 {
        return HashSet::from([position.clone()]);
    }

    get_neighbours(position)
        .into_iter()
        .map(|p| (p, map_lookup.get(&p)))
        .filter(|(_, v)| *v == Some(&(position_value + 1)))
        .flat_map(|(p, _)| get_trailends_reachable(&p, map_lookup))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(Span::new(input)).unwrap().1;

    let map_lookup: HashMap<IVec2, u32> = HashMap::from_iter(map.iter().flatten().map(|sq| {
        (
            IVec2::new(
                sq.position.location_line() as i32 - 1,
                sq.position.get_column() as i32 - 1,
            ),
            sq.value,
        )
    }));

    let zero_positions: Vec<IVec2> = map_lookup
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(k, _)| k)
        .cloned()
        .collect();

    let mut total_score = 0;
    for zero_position in &zero_positions {
        total_score += get_trailends_reachable(zero_position, &map_lookup).len();
    }

    Some(total_score as u32)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
