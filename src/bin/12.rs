use std::collections::{BTreeSet, HashMap, VecDeque};

use indexmap::IndexMap;
use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .filter_map(|line| match line {
            "" => None,
            _ => Some(line.chars().collect()),
        })
        .collect();

    let mut grouped = HashMap::new();

    for (coords, row) in grid.iter().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            let coords = (coords, col_idx);
            grouped
                .entry(c)
                .and_modify(|el: &mut Vec<(usize, usize)>| el.push(coords))
                .or_insert(vec![coords]);
        }
    }

    let mut s = 0usize;

    for (_, coords) in grouped {
        let chunks = chunk_coords(coords.into_iter().collect());

        s += chunks.into_iter().map(fence_price).sum::<usize>();
    }
    Some(s as u32)
}

fn is_neighbouring((a_row, a_col): &(usize, usize), (b_row, b_col): &(usize, usize)) -> bool {
    (*a_row as i64 - *b_row as i64).abs() + (*a_col as i64 - *b_col as i64).abs() == 1
}

fn chunk_coords(mut coords: BTreeSet<(usize, usize)>) -> Vec<BTreeSet<(usize, usize)>> {
    match coords.pop_first() {
        None => vec![BTreeSet::new()],
        Some(coord) => {
            let chunk = find_first_chunk(coords.clone(), coord);
            let remaining_plants: BTreeSet<(usize, usize)> =
                coords.difference(&chunk).cloned().collect();

            [vec![chunk], (chunk_coords(remaining_plants))].concat()
        }
    }
}

fn find_first_chunk(
    mut coords: BTreeSet<(usize, usize)>,
    coord: (usize, usize),
) -> BTreeSet<(usize, usize)> {
    let mut chunk = BTreeSet::new();
    let mut q = VecDeque::new();
    q.push_front(coord);

    while !q.is_empty() {
        let c = q.pop_front().unwrap();
        let ns = neighbours(c)
            .into_iter()
            .flatten()
            .filter_map(|n| coords.take(&n));
        q.extend(ns);
        chunk.insert(c);
    }

    chunk
}

fn fence_price(coords: BTreeSet<(usize, usize)>) -> usize {
    let neighbour_sides = coords
        .iter()
        .combinations(2)
        .filter(|l| is_neighbouring(l[0], l[1]))
        .count();

    ((coords.len() * 4) - 2 * neighbour_sides) * coords.len()
}

fn neighbours((row, col): (usize, usize)) -> [Option<(usize, usize)>; 4] {
    let mut res = [None; 4];
    if row != 0 {
        res[0] = Some((row - 1, col))
    }
    if col != 0 {
        res[1] = Some((row, col - 1))
    }
    res[2] = Some((row + 1, col));
    res[3] = Some((row, col + 1));
    res
}

// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// impl Direction {
//     fn opposite(&self) -> Self {
//         match self {
//             Direction::Up => Self::Down,
//             Direction::Down => Self::Up,
//             Direction::Left => Self::Right,
//             Direction::Right => Self::Left,
//         }
//     }
// }

fn fence_price_2(coords: BTreeSet<(usize, usize)>) -> usize {
    // let region: IndexMap<(usize, usize), Vec<(usize, usize)>> = coords
    //     .iter()
    //     .map(|el| {
    //         let neighbors = coords
    //             .iter()
    //             .filter_map(|other| match is_neighbouring(el, other) {
    //                 true => {
    //                     if el == other {
    //                         None
    //                     } else {
    //                         Some(*other)
    //                     }
    //                 }
    //                 false => None,
    //             })
    //             .collect::<Vec<(usize, usize)>>();
    //         (*el, neighbors)
    //     })
    //     .collect();

    // ((coords.len() * 4) - 2 * neighbor_sides.len()) * coords.len()

    let mut corners = 0;

    for (r, c) in coords.clone() {
        // (bottom_right, top_right, top_left, bottom_left)
        for (r, c) in [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)] {
            corners += match get_around_corner((r, c), &coords) {
                (true, true, true, true) | (false, false, false, false) => 0,
                (true, true, true, false)
                | (true, true, false, true)
                | (true, false, true, true)
                | (false, true, true, true)
                | (false, false, false, true)
                | (false, false, true, false)
                | (false, true, false, false)
                | (true, false, false, false) => 1,
                (true, true, false, false)
                | (false, false, true, true)
                | (false, true, true, false)
                | (true, false, false, true) => 0,
                (true, false, true, false) | (false, true, false, true) => 2,
            };
        }
    }

    println!("{}", corners);
    corners as usize
}

type Coord = (usize, usize);

fn get_around_corner((r, c): Coord, coords: &BTreeSet<Coord>) -> (bool, bool, bool, bool) {
    let r_nonzero = r != 0;
    let c_nonzero = c != 0;

    let bottom_right = coords.contains(&(r, c));
    let top_right = if r_nonzero {
        coords.contains(&(r - 1, c))
    } else {
        false
    };
    let top_left = if r_nonzero && c_nonzero {
        coords.contains(&(r - 1, c - 1))
    } else {
        false
    };

    let bottom_left = if c_nonzero {
        coords.contains(&(r, c - 1))
    } else {
        false
    };

    (bottom_right, top_right, top_left, bottom_left)
}

// fn get_turns(
//     initial_pos: (usize, usize),
//     current_pos: (usize, usize),
//     current_dir: Direction,
//     ns: IndexMap<(usize, usize), Vec<(usize, usize)>>,
// ) -> usize {
//     todo!()
// }

// fn get_dirs(
//     (pos_r, pos_c): (usize, usize),
//     ns: Vec<(usize, usize)>,
// ) -> VecDeque<(Direction, (usize, usize))> {
//     ns.iter()
//         .map(|n| {
//             let dir = match *n {
//                 (r, c) if r == pos_r && c == pos_c + 1 => Direction::Left,
//                 (r, c) if r == pos_r + 1 && c == pos_c => Direction::Down,
//                 (r, c) if r == pos_r && c == pos_c - 1 => Direction::Right,
//                 (r, c) if r == pos_r - 1 && c == pos_c => Direction::Up,
//                 _ => unimplemented!(),
//             };
//             (dir, *n)
//         })
//         .collect()
// }

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .filter_map(|line| match line {
            "" => None,
            _ => Some(line.chars().collect()),
        })
        .collect();

    let mut grouped = HashMap::new();

    for (coords, row) in grid.iter().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            let coords = (coords, col_idx);
            grouped
                .entry(c)
                .and_modify(|el: &mut Vec<(usize, usize)>| el.push(coords))
                .or_insert(vec![coords]);
        }
    }

    let mut s = 0usize;

    for (ch, coords) in grouped {
        let chunks = chunk_coords(coords.into_iter().collect());
        println!("{}", ch);
        s += chunks.into_iter().map(fence_price_2).sum::<usize>();
    }

    Some(s as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
