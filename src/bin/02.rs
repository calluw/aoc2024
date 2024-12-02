use nom::{
    character::complete::{digit1, multispace1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use std::cmp::min;
use std::str::FromStr;

advent_of_code::solution!(2);

type Report = Vec<u32>;

fn parse_report(input: &str) -> IResult<&str, Report> {
    separated_list1(multispace1, map_res(digit1, u32::from_str))(input)
}

fn get_reports(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(parse_report)
        .map(|r| r.expect("puzzle input to parse").1)
        .collect::<Vec<_>>()
}

fn get_diffs(report: &Report) -> Vec<i32> {
    let mut diffs = Vec::new();
    for window in report.windows(2) {
        if let &[first, second] = window {
            diffs.push(second as i32 - first as i32)
        }
    }
    diffs
}

fn diffs_are_safe(diffs: &[i32]) -> bool {
    let signs_match = diffs.iter().all(|d| *d >= 0) || diffs.iter().all(|d| *d < 0);
    let right_magnitude = diffs.iter().all(|d| d.abs() >= 1 && d.abs() <= 3);

    signs_match && right_magnitude
}

fn report_is_safe_p1(report: &Report) -> bool {
    diffs_are_safe(&get_diffs(report))
}

fn report_is_safe_p2(report: &Report) -> bool {
    let diffs = get_diffs(report);

    if diffs_are_safe(&diffs) {
        return true;
    }

    // Discard those with too many wrong magnitudes
    let wrong_magnitude_count = diffs.iter().filter(|d| d.abs() < 1 || d.abs() > 3).count();
    if wrong_magnitude_count > 2 {
        // Cannot be fixed (1 bad level can cause max two bad diffs, consecutive)
        return false;
    }

    // Discard those with too many wrong signs
    let positive_sign_count = diffs.iter().filter(|d| **d >= 0).count();
    let wrong_sign_count = min(positive_sign_count, diffs.len() - positive_sign_count);
    if wrong_sign_count > 2 {
        // Cannot be fixed (1 bad level can cause max two bad diffs, consecutive)
        return false;
    }

    // Try removing each level from the report in turn to check if the report is now safe
    let mut changed_report = report.clone();
    for idx in 0..report.len() {
        changed_report.remove(idx);

        if diffs_are_safe(&get_diffs(&changed_report)) {
            return true;
        }

        changed_report = report.clone();
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        get_reports(input)
            .into_iter()
            .filter(report_is_safe_p1)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        get_reports(input)
            .into_iter()
            .filter(report_is_safe_p2)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
