use std::{collections::HashMap, ops::Add, u64};

advent_of_code::solution!(14);

type Coords = (i64, i64);

fn parse(input: &str) -> Vec<(Coords, Coords)> {
    input
        .trim()
        .split("\n")
        .map(|ln| {
            let parts = ln.split(" ").collect::<Vec<_>>();
            let pos: Vec<i64> = parts[0][2..]
                .split(",")
                .map(|el| el.parse().unwrap())
                .collect();

            let vels: Vec<i64> = parts[1][2..]
                .split(",")
                .map(|el| el.parse().unwrap())
                .collect();

            ((pos[1], pos[0]), (vels[1], vels[0]))
        })
        .collect()
}

// const WIDTH: i64 = 11;
const WIDTH: i64 = 101;
// const HEIGHT: i64 = 7;
const HEIGHT: i64 = 103;
const SECONDS: i64 = 100;
pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);

    let safety_rating = input
        .into_iter()
        .map(|(coords, velocity)| {
            let (y, x) = coords;
            let (vy, vx) = velocity;
            let y = add_with_custom_overflow(y, vy, HEIGHT, SECONDS);
            let x = add_with_custom_overflow(x, vx, WIDTH, SECONDS);

            get_quad((y, x), WIDTH, HEIGHT)
        })
        .fold(HashMap::new(), |mut acc, n| {
            if n != 0 {
                acc.entry(n)
                    .and_modify(|cnt: &mut u64| *cnt += 1)
                    .or_insert(1);
            }
            acc
        })
        .values()
        .product();

    Some(safety_rating)
}

/// return the quadrant of the coord.
/// 0 if on the center axises or:
/// ```
/// 1 | 2
/// --+--
/// 3 | 4
/// ```
fn get_quad((y, x): Coords, w: i64, h: i64) -> usize {
    let h_mid = (h - 1) / 2;
    let v_mid = (w - 1) / 2;

    match (y.cmp(&h_mid), x.cmp(&v_mid)) {
        (_, std::cmp::Ordering::Equal) | (std::cmp::Ordering::Equal, _) => 0,
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => 1,
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => 2,
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => 3,
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => 4,
    }
}

fn add_with_custom_overflow(n: i64, d: i64, overflow: i64, seconds: i64) -> i64 {
    let res = n + seconds * d;
    (res % overflow + overflow) % overflow
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);
    let mut min_sr = u64::MAX;
    let mut min_sr_sec = 0;

    for ii in 0..100_000u64 {
        let safety_rating: u64 = input
            .clone()
            .into_iter()
            .map(|(coords, velocity)| {
                let (y, x) = coords;
                let (vy, vx) = velocity;
                let y = add_with_custom_overflow(y, vy, HEIGHT, ii.try_into().unwrap());
                let x = add_with_custom_overflow(x, vx, WIDTH, ii.try_into().unwrap());

                get_quad((y, x), WIDTH, HEIGHT)
            })
            .fold(HashMap::new(), |mut acc, n| {
                if n != 0 {
                    acc.entry(n)
                        .and_modify(|cnt: &mut u64| *cnt += 1)
                        .or_insert(1);
                }
                acc
            })
            .values()
            .product();

        if safety_rating < min_sr {
            min_sr = safety_rating;
            min_sr_sec = ii;
        }
    }

    Some(min_sr_sec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_quad() {
        assert_eq!(get_quad((3, 3), 11, 7), 0);
        assert_eq!(get_quad((1, 1), 11, 7), 1);
        assert_eq!(get_quad((1, 9), 11, 7), 2);
        assert_eq!(get_quad((5, 1), 11, 7), 3);
        assert_eq!(get_quad((5, 9), 11, 7), 4);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2549));
    }
}
