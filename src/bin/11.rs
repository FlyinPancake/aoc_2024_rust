advent_of_code::solution!(11);

use anyhow::{bail, Result};
use cached::proc_macro::cached;
use itertools::Itertools;
use std::{collections::HashMap, num::ParseIntError};

pub fn parse(input: &str) -> Vec<u64> {
    let res: Result<Vec<u64>, ParseIntError> = input
        .trim()
        .split(" ")
        .map(|el| el.parse::<u64>())
        .collect();
    res.unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut nums = parse(input).into_iter().counts();

    for _ in 0..25 {
        nums = process_nums(nums)
    }
    Some(nums.values().sum::<usize>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut nums = parse(input).into_iter().counts();

    for _ in 0..75 {
        nums = process_nums(nums)
    }
    Some(nums.values().sum::<usize>() as u64)
}

fn process_nums(counts: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_counts = HashMap::new();
    for (k, v) in counts {
        for r in blink(k) {
            *new_counts.entry(r).or_insert(0) += v
        }
    }

    new_counts
}

#[cached]
fn blink(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![1];
    }
    match split_num(n) {
        Ok((l, r)) => vec![l, r],
        Err(_) => vec![n * 2024],
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
