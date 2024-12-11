advent_of_code::solution!(11);

use anyhow::{bail, Result};
use cached::proc_macro::cached;
use std::num::ParseIntError;

pub fn parse(input: &str) -> Vec<u64> {
    let res: Result<Vec<u64>, ParseIntError> = input
        .trim()
        .split(" ")
        .map(|el| el.parse::<u64>())
        .collect();
    res.unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = parse(input).into_iter().map(|n| blink(n, 25)).sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = parse(input).into_iter().map(|n| blink(n, 75)).sum();
    Some(res)
}

#[cached]
fn blink(n: u64, depth: u64) -> u64 {
    if depth == 0 {
        return 1;
    }

    if n == 0 {
        return blink(1, depth - 1);
    }
    match split_num(n) {
        Ok((l, r)) => blink(l, depth - 1) + blink(r, depth - 1),
        Err(_) => blink(n * 2024, depth - 1),
    }
}

fn split_num(n: u64) -> Result<(u64, u64)> {
    let mut s = n.to_string();
    if s.len() % 2 != 0 {
        bail!("not symmetric")
    }

    let tail = s.split_off(s.len() / 2);

    Ok((s.parse()?, tail.parse()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
