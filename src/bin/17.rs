use std::str::FromStr;

use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Debug)]
enum Operator {
    ADivide = 0,
    BXorLiteral = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<i64> for Operator {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::ADivide,
            1 => Self::BXorLiteral,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
        }
    }
}

#[derive(Debug)]
enum Operand {
    Value(i64), // 1-3
    RegA,       // 4
    RegB,       //5
    RegC,       //6
}

impl From<i64> for Operand {
    fn from(value: i64) -> Self {
        match value {
            v if v < 1 => unimplemented!(),
            v if v <= 3 => Self::Value(v),
            4 => Self::RegA,
            5 => Self::RegB,
            6 => Self::RegC,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,

    program: Vec<(i64, i64)>,
    program_pointer: usize,
    out: Vec<i64>,
}

impl FromStr for Computer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim().split('\n');
        let reg_a = s.next().unwrap()[12..].parse()?;
        let reg_b = s.next().unwrap()[12..].parse()?;
        let reg_c = s.next().unwrap()[12..].parse()?;

        let program = s.nth(1).unwrap()[9..]
            .split(',')
            .map(|el| i64::from_str(el).unwrap())
            .chunks(2)
            .into_iter()
            .map(|mut el| (el.next().unwrap(), el.next().unwrap()))
            .collect();

        Ok(Computer {
            reg_a,
            reg_b,
            reg_c,
            program,
            program_pointer: 0,
            out: vec![],
        })
    }
}

impl Computer {
    fn perform_operation(&mut self) {}
}

pub fn part_one(input: &str) -> Option<u32> {
    let computer: Computer = input.parse().unwrap();

    eprintln!("{:?}", computer);
    todo!();
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
