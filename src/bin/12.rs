use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .into_iter()
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
        s += fence_price(chunks);
    }
    Some(s as u32)
}

fn is_neighbouring((a_row, a_col): &(usize, usize), (b_row, b_col): &(usize, usize)) -> bool {
    (*a_row as i64 - *b_row as i64).abs() + (*a_col as i64 - *b_col as i64).abs() == 1
}

fn chunk_coords(mut coords: BTreeSet<(usize, usize)>) -> BTreeSet<(usize, usize)> {
    match coords.pop_first() {
        None => BTreeSet::new(),
        Some(coord) => {
            let neighbours = coords
                .clone()
                .into_iter()
                .enumerate()
                .filter_map(|(idx, el)| match is_neighbouring(&coord, &el) {
                    true => Some(el),
                    false => None,
                })
                .rev()
                .collect::<BTreeSet<_>>();

            coords = coords.difference(&neighbours).cloned().collect();

            let neighbours = neighbours.into_iter().flat_map(|n| {
                let mut coords = coords.clone();
                coords.insert(n);
                chunk_coords(coords)
            });

            let mut chunk = BTreeSet::new();
            chunk.insert(coord);
            for n in neighbours {
                chunk.insert(n);
            }

            chunk
        }
    }
}

fn fence_price(coords: BTreeSet<(usize, usize)>) -> usize {
    let neighbour_sides = coords
        .iter()
        .combinations(2)
        .filter(|l| is_neighbouring(l[0], l[1]))
        .count();

    ((coords.len() * 4) - neighbour_sides) * coords.len()
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
