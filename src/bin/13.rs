use itertools::*;

advent_of_code::solution!(13);

type Coord = (i64, i64);

#[derive(Debug)]
struct ClawMachine {
    pub btn_a: Coord,
    pub btn_b: Coord,
    pub prize: Coord,
}

fn parse(input: &str) -> Vec<ClawMachine> {
    let chunked: Vec<_> = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .chunks(3)
        .into_iter()
        .map(|mut raw| {
            let button_a = raw.next().unwrap();
            let button_b = raw.next().unwrap();
            let prize = raw.next().unwrap();

            let btn_ax = button_a[12..14].parse().unwrap();
            let btn_ay = button_a[18..20].parse().unwrap();
            let btn_a: Coord = (btn_ax, btn_ay);

            let btn_b: Coord = (
                button_b[12..14].parse().unwrap(),
                button_b[18..20].parse().unwrap(),
            );

            let xeq = prize.find("X=").unwrap();
            let comma = prize.find(",").unwrap();
            let yeq = prize.find("Y=").unwrap();

            let prize: Coord = (
                prize[xeq + 2..comma].parse().unwrap(),
                prize[yeq + 2..].parse().unwrap(),
            );

            ClawMachine {
                btn_a,
                btn_b,
                prize,
            }
        })
        .collect();

    chunked
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);

    let mut sum = 0;

    for cm in input {
        sum += push_buttons(cm);
    }

    Some(sum as u32)
}

fn push_buttons(claw: ClawMachine) -> i64 {
    let ClawMachine {
        btn_a,
        btn_b,
        prize,
    } = claw;

    let (x1, y1) = (btn_a.0 as f64, btn_a.1 as f64);
    let (x2, y2) = (btn_b.0 as f64, btn_b.1 as f64);
    let (px, py) = (prize.0 as f64, prize.1 as f64);

    let b = (py * x1 - y1 * px) / (y2 * x1 - y1 * x2);

    let a = (px - b * x2) / x1;

    let sol = [a, b];

    for s0 in [sol[0].ceil(), sol[0].floor()] {
        for s1 in [sol[1].ceil(), sol[1].floor()] {
            let eq1_ok = x1 * s0 + x2 * s1 == px;
            let eq2_ok = y1 * s0 + y2 * s1 == py;
            if eq1_ok && eq2_ok {
                return (sol[0] * 3f64 + sol[1]) as i64;
            }
        }
    }

    0
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);

    let input = input.into_iter().map(|mut el| {
        el.prize.0 += 10_000_000_000_000;
        el.prize.1 += 10_000_000_000_000;
        el
    });

    let mut sum = 0;

    for cm in input {
        sum += push_buttons(cm);
    }

    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
