use std::collections::{BTreeMap, BTreeSet};

use nom::{
    bytes::complete::tag,
    character::complete::{self, char},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(5);

fn parse_rules(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(
        char('\n'),
        separated_pair(complete::u32, char('|'), complete::u32),
    )(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(char('\n'), separated_list1(char(','), complete::u32))(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    separated_pair(parse_rules, tag("\n\n"), parse_updates)(input)
}

fn get_invalid_index(
    update: &[u32],
    map_after_to_befores: &BTreeMap<u32, BTreeSet<u32>>,
) -> Option<usize> {
    let mut befores: BTreeSet<u32> = BTreeSet::new();
    let mut invalid_index = None;
    for (idx, page) in update.into_iter().enumerate() {
        if befores.contains(page) {
            // This page should have been before a page which has already been seen, and so this whole update is invalid
            invalid_index = Some(idx);
            break;
        }
        befores.extend(map_after_to_befores.get(page).unwrap_or(&BTreeSet::new()));
    }
    invalid_index
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input).expect("puzzle input to parse").1;

    let mut map_after_to_befores = BTreeMap::new();

    for rule in rules {
        map_after_to_befores
            .entry(rule.1)
            .or_insert(BTreeSet::new())
            .insert(rule.0);
    }

    let mut middles = Vec::new();
    for update in updates {
        if get_invalid_index(&update, &map_after_to_befores).is_none() {
            middles.push(update[update.len() / 2])
        }
    }

    Some(middles.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input).expect("puzzle input to parse").1;

    let mut map_after_to_befores = BTreeMap::new();

    for rule in rules {
        map_after_to_befores
            .entry(rule.1)
            .or_insert(BTreeSet::new())
            .insert(rule.0);
    }

    let mut middles = Vec::new();
    for mut update in updates {
        let mut invalid_index = get_invalid_index(&update, &map_after_to_befores);
        if invalid_index.is_none() {
            // It's valid, don't want to use it
            continue;
        }

        // Try shifting invalid values back so that they are before others, which the rules may say
        // they have to be. Do this regardless of which element is invalid, so this handles
        // naturally there being more than one invalid.
        while invalid_index.is_some() {
            let invalid_page = update.remove(invalid_index?);
            // NOTE: The invalid index can never be 0, since can't proved at index 0 the update is invalid
            update.insert(invalid_index? - 1, invalid_page);
            invalid_index = get_invalid_index(&update, &map_after_to_befores);
        }

        middles.push(update[update.len() / 2]);
    }

    Some(middles.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
