use std::collections::{HashMap, HashSet};

use glam::IVec2;
use nom::{
    branch::alt,
    character::complete::char,
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};

advent_of_code::solution!(6);

#[derive(Debug, Clone)]
enum Tile {
    Floor,
    Obstacle,
    Guard,
}

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    alt((
        value(Tile::Floor, char('.')),
        value(Tile::Obstacle, char('#')),
        value(Tile::Guard, char('^')),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(char('\n'), many1(parse_tile))(input)
}

fn determine_guard_path(
    mut guard_pos: IVec2,
    grid_width: i32,
    grid_height: i32,
    mut guard_direction: IVec2,
    grid_lookup: HashMap<IVec2, Tile>,
) -> (HashSet<(IVec2, IVec2)>, bool) {
    let mut guard_path = HashSet::new();
    let mut hit_loop = false;
    while guard_pos.x >= 0
        && guard_pos.y >= 0
        && guard_pos.x < grid_width
        && guard_pos.y < grid_height
    {
        if guard_path.contains(&(guard_pos, guard_direction)) {
            // Guard has been here facing the same way before: they are in a loop!
            hit_loop = true;
            break;
        }

        guard_path.insert((guard_pos, guard_direction));
        let next_pos = guard_pos + guard_direction;
        // If its not in the lookup, it's off the grid, so just pretend it's floor
        let next_tile = grid_lookup.get(&next_pos).unwrap_or(&Tile::Floor);

        match next_tile {
            // Guard starting position is just another floor tile
            Tile::Floor | Tile::Guard => (),
            // Rotation 90 degrees clockwise is (y, -x)
            Tile::Obstacle => guard_direction = IVec2::new(guard_direction.y, -guard_direction.x),
        }

        guard_pos += guard_direction;
    }

    (guard_path, hit_loop)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input).expect("puzzle input to parse").1;
    let grid_width = grid[0].len() as i32;
    let grid_height = grid.len() as i32;

    let mut grid_lookup = HashMap::new();

    let mut guard_pos = None;
    for (i, row) in grid.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            grid_lookup.insert(IVec2::new(i as i32, j as i32), tile.clone());
            if let Tile::Guard = tile {
                guard_pos = Some(IVec2::new(i as i32, j as i32));
            }
        }
    }

    let guard_pos = guard_pos.expect("guard to be in the initial grid");
    let guard_direction = IVec2::new(-1, 0);

    let (guard_path, hit_loop) = determine_guard_path(
        guard_pos,
        grid_width,
        grid_height,
        guard_direction,
        grid_lookup,
    );

    match hit_loop {
        true => None,
        false => Some(HashSet::<IVec2>::from_iter(guard_path.iter().map(|x| x.0)).len() as u32),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input).expect("puzzle input to parse").1;
    let grid_width = grid[0].len() as i32;
    let grid_height = grid.len() as i32;

    let mut grid_lookup = HashMap::new();

    let mut guard_pos = None;
    for (i, row) in grid.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            grid_lookup.insert(IVec2::new(i as i32, j as i32), tile.clone());
            if let Tile::Guard = tile {
                guard_pos = Some(IVec2::new(i as i32, j as i32));
            }
        }
    }

    let guard_pos = guard_pos.expect("guard to be in the initial grid");

    // For every single floor tile, try it as an obstacle and see if it hits a loop
    let mut loop_obstacle_pos = HashSet::new();
    for (pos, tile) in &grid_lookup {
        match tile {
            // Not allowed to put an obstacle on original guard post
            Tile::Obstacle | Tile::Guard => continue,
            Tile::Floor => (),
        }

        let mut grid_lookup = grid_lookup.clone();
        let guard_direction = IVec2::new(-1, 0);
        grid_lookup.insert(*pos, Tile::Obstacle);
        let (_, hit_loop) = determine_guard_path(
            guard_pos,
            grid_width,
            grid_height,
            guard_direction,
            grid_lookup,
        );

        if hit_loop {
            loop_obstacle_pos.insert(pos);
        }
    }

    Some(loop_obstacle_pos.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
