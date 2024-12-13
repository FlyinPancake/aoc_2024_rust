use itertools::*;
use ndarray::prelude::*;
use ndarray_linalg::*;

advent_of_code::solution!(13);

type Coord = (i64, i64);

fn coord_sub((bx, by): Coord, (sx, sy): Coord) -> Coord {
    (bx - sx, by - sy)
}

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

    let lhs: Array2<f64> = arr2(&[
        [btn_a.0 as f64, btn_b.0 as f64],
        [btn_a.1 as f64, btn_b.1 as f64],
    ]);

    // lhs[[0, 0]] = btn_a.0 as f64;
    // lhs[[0, 1]] = btn_b.0 as f64;
    // lhs[[1, 0]] = btn_a.1 as f64;
    // lhs[[1, 1]] = btn_b.1 as f64;

    let rhs: Array1<f64> = arr1(&[prize.0 as f64, prize.1 as f64]);
    // rhs[[0, 0]] = prize.0 as f64;
    // rhs[[1, 0]] = prize.1 as f64;

    let sol = lhs.solve(&rhs).unwrap(); //.dot(&rhs);

    let sol = sol.map(|el| round(*el, 3));

    if sol[0].fract() > 0.001 || sol[1].fract() > 0.001 {
        eprintln!("x:{:.3} y: {:.3}", sol[0].fract(), sol[1].fract());
        return 0;
    }

    (sol[0] * 3f64 + sol[1]) as i64
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);

    let input = input.into_iter().map(|mut el| {
        el.prize.0 += 10000000000000;
        el.prize.1 += 10000000000000;
        el
    });

    let mut sum = 0;

    for cm in input {
        sum += push_buttons(cm);
    }

    Some(sum as u32)
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
