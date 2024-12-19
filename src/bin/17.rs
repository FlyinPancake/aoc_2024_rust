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
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Operand {
    Value(i64), // 0-3
    RegA,       // 4
    RegB,       // 5
    RegC,       // 6
}

impl From<i64> for Operand {
    fn from(value: i64) -> Self {
        match value {
            v if v < 0 => unimplemented!(),
            v if v <= 3 => Self::Value(v),
            4 => Self::RegA,
            5 => Self::RegB,
            6 => Self::RegC,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Default)]
struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,

    program: Vec<i64>,
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

        let program: Vec<i64> = s.nth(1).unwrap()[9..]
            .split(',')
            .map(|el| i64::from_str(el).unwrap())
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
    fn get_operand(&self, operand: &Operand) -> i64 {
        match operand {
            Operand::Value(v) => *v,
            Operand::RegA => self.reg_a,
            Operand::RegB => self.reg_b,
            Operand::RegC => self.reg_c,
        }
    }
    fn perform_operation(&mut self) {
        let operator = &self.program[self.program_pointer];
        let operator = Operator::from(*operator);

        let operand = &self.program[self.program_pointer + 1];

        match operator {
            Operator::ADivide => {
                let operand = self.get_operand(&(*operand).into());
                let res = self.reg_a / 2i64.pow(operand as u32);
                self.reg_a = res;
            }
            Operator::BXorLiteral => {
                // let operand = self.get_operand(&(*operand).into());

                self.reg_b ^= operand;
            }
            Operator::Bst => {
                let operand = self.get_operand(&(*operand).into());
                self.reg_b = operand.rem_euclid(8);
            }
            Operator::Jnz => {
                if self.reg_a != 0 {
                    let operand = self.get_operand(&(*operand).into());
                    self.program_pointer = operand as usize;
                    return;
                }
            }
            Operator::Bxc => {
                self.reg_b ^= self.reg_c;
            }
            Operator::Out => {
                let operand = self.get_operand(&(*operand).into());
                self.out.push(operand.rem_euclid(8));
            }
            Operator::Bdv => {
                let operand = self.get_operand(&(*operand).into());
                self.reg_b = self.reg_a / 2i64.pow(operand as u32);
            }
            Operator::Cdv => {
                let operand = self.get_operand(&(*operand).into());
                self.reg_c = self.reg_a / 2i64.pow(operand as u32);
            }
        }
        self.program_pointer += 2;
    }

    fn run_to_end(&mut self) {
        while self.program.len() > self.program_pointer {
            self.perform_operation();
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer: Computer = input.parse().unwrap();

    computer.run_to_end();
    let res = computer.out.into_iter().join(",");

    Some(res)
}

pub fn part_two(_input: &str) -> Option<i64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4635635210".to_owned()));
    }

    #[test]
    fn test_computer_part_one() {
        let mut computer = Computer {
            program: vec![2, 6],
            reg_c: 9,
            ..Default::default()
        };
        computer.run_to_end();
        assert_eq!(computer.reg_b, 1);

        let mut computer = Computer {
            reg_a: 10,
            program: vec![5, 0, 5, 1, 5, 4],
            ..Default::default()
        };
        computer.run_to_end();
        assert_eq!(computer.out, vec![0, 1, 2]);

        let mut computer = Computer {
            reg_a: 2024,
            program: vec![0, 1, 5, 4, 3, 0],
            ..Default::default()
        };
        computer.run_to_end();
        assert_eq!(computer.out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.reg_a, 0);

        let mut computer = Computer {
            reg_b: 29,
            program: vec![1, 7],
            ..Default::default()
        };
        computer.run_to_end();
        assert_eq!(computer.reg_b, 26);

        let mut computer = Computer {
            reg_b: 2024,
            reg_c: 43690,
            program: vec![4, 0],
            ..Default::default()
        };
        computer.run_to_end();
        assert_eq!(computer.reg_b, 44354);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
