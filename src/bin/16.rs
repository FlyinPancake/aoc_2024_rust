use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(16);

fn parse(input: &str) -> HashMap<(usize, usize), char> {
    input
        .trim()
        .split('\n')
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(move |(col_idx, ch)| ((row_idx, col_idx), ch))
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    const fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }

    fn turn_cost(&self, other: &Direction) -> i64 {
        if self == other {
            return 0;
        }
        if self.opposite() == *other {
            return 2000;
        }
        1000
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Node {
    position: (usize, usize),
    direction: Direction,
}

impl Node {
    fn moved_to_dir(&self) -> Self {
        let (r, c) = self.position;
        let position = match self.direction {
            Direction::North => (r - 1, c),
            Direction::West => (r, c - 1),
            Direction::South => (r + 1, c),
            Direction::East => (r, c + 1),
        };
        Self {
            position,
            direction: self.direction,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    cost: i64,
    node: Node,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    #[cfg(debug_assertions)]
    let height = input.trim().split("\n").count();
    #[cfg(debug_assertions)]
    let width = input.trim().split("\n").next().unwrap().len();
    let input = parse(input);
    let start = input.iter().find(|(_, ch)| **ch == 'S').unwrap();
    #[cfg(debug_assertions)]
    let end = input.iter().find(|(_, ch)| **ch == 'E').unwrap();

    let mut queue = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut prev = HashMap::new();

    queue.push(State {
        cost: 0,
        node: Node {
            position: *start.0,
            direction: Direction::East,
        },
    });

    for (k, v) in input.iter() {
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            distances.insert(
                Node {
                    position: *k,
                    direction: dir,
                },
                i64::MAX,
            );
            // queue.push_back(k);
        }
    }

    distances
        .entry(Node {
            position: *start.0,
            direction: Direction::East,
        })
        .insert_entry(0);

    let mut tc = None;

    'outer: while let Some(coords) = queue.pop() {
        let base_cost = match input[&coords.node.position] {
            '#' => continue,
            'E' => {
                tc = Some(coords.cost);
                break 'outer;
            }
            _ => distances[&coords.node],
        };

        for dir in [
            Direction::West,
            Direction::South,
            Direction::East,
            Direction::North,
        ] {
            if dir == coords.node.direction {
                continue;
            }
            let state_turned_node = Node {
                direction: dir,
                position: coords.node.position,
            };
            let cost = base_cost + dir.turn_cost(&coords.node.direction);
            if cost < distances[&state_turned_node] {
                distances.entry(state_turned_node).insert_entry(cost);
                prev.entry(state_turned_node).insert_entry(coords.node);
                queue.push(State {
                    cost,
                    node: state_turned_node,
                });
            }
        }
        let next_pos = coords.node.moved_to_dir();
        let cost = base_cost + 1;

        if cost < distances[&next_pos] {
            distances.entry(next_pos).insert_entry(cost);
            prev.entry(next_pos).insert_entry(coords.node);
            queue.push(State {
                cost,
                node: next_pos,
            });
        }
    }

    // tc = distances
    //     .iter()
    //     .filter(|el| el.0.position == *end.0)
    //     .map(|el| el.1)
    //     .min()
    //     .cloned();

    if tc.is_none() {
        panic!("No route found")
    }

    #[cfg(debug_assertions)]
    {
        let mut seq = VecDeque::new();

        let mut u = prev
            .keys()
            .find(|Node { position, .. }| *position == *end.0)
            .unwrap()
            .to_owned();

        while u.position != *start.0 {
            seq.push_front(u);
            u = prev[&u]
        }

        // dbg!(seq);
        for r in 0..height {
            for c in 0..width {
                let ch = match seq.iter().find(|el| el.position == (r, c)) {
                    Some(Node { direction, .. }) => match direction {
                        Direction::North => '^',
                        Direction::West => '<',
                        Direction::South => 'v',
                        Direction::East => '>',
                    },
                    None => input[&(r, c)],
                };
                eprint!("{}", ch);
            }
            eprintln!()
        }
    }

    tc
}

const fn neighbors(node: Node) -> [((usize, usize), Direction); 4] {
    let Node {
        position: (row, col),
        ..
    } = node;
    [
        ((row, col + 1), Direction::East),
        ((row - 1, col), Direction::North),
        ((row, col - 1), Direction::West),
        ((row + 1, col), Direction::South),
    ]
}

pub fn part_two(input: &str) -> Option<i64> {
    let height = input.trim().split("\n").count();
    let width = input.trim().split("\n").next().unwrap().len();
    let input = parse(input);
    let start = input.iter().find(|(_, ch)| **ch == 'S').unwrap();
    let end = input.iter().find(|(_, ch)| **ch == 'E').unwrap();

    let mut queue = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut prev = HashMap::new();

    queue.push(State {
        cost: 0,
        node: Node {
            position: *start.0,
            direction: Direction::East,
        },
    });

    for (k, _) in input.iter() {
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            distances.insert(
                Node {
                    position: *k,
                    direction: dir,
                },
                i64::MAX,
            );
        }
    }

    distances
        .entry(Node {
            position: *start.0,
            direction: Direction::East,
        })
        .insert_entry(0);

    'outer: while let Some(coords) = queue.pop() {
        let base_cost = match input[&coords.node.position] {
            '#' => continue,

            'E' => {
                match coords.cost.cmp(&distances[&coords.node]) {
                    std::cmp::Ordering::Less => {}
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => {
                        break 'outer;
                    }
                }
                continue;
            }

            _ => distances[&coords.node],
        };
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            if dir == coords.node.direction {
                continue;
            }
            let self_turned = Node {
                position: coords.node.position,
                direction: dir,
            };
            let cost = base_cost + dir.turn_cost(&coords.node.direction);

            match cost.cmp(&distances[&self_turned]) {
                std::cmp::Ordering::Less => {
                    distances.entry(self_turned).insert_entry(cost);

                    prev.entry(self_turned)
                        .insert_entry(HashSet::from([coords.node]));
                    queue.push(State {
                        cost,
                        node: self_turned,
                    });
                }
                std::cmp::Ordering::Equal => {
                    prev.entry(self_turned).and_modify(|v| {
                        v.insert(coords.node);
                    });
                }
                std::cmp::Ordering::Greater => {
                    continue;
                }
            }
        }

        let next_pos = coords.node.moved_to_dir();

        let cost = base_cost + 1;

        match cost.cmp(&distances[&next_pos]) {
            std::cmp::Ordering::Less => {
                distances.entry(next_pos).insert_entry(cost);
                prev.entry(next_pos)
                    .insert_entry(HashSet::from([coords.node]));
                queue.push(State {
                    cost,
                    node: next_pos,
                });
            }
            std::cmp::Ordering::Equal => {
                prev.entry(next_pos).and_modify(|v| {
                    v.insert(coords.node);
                });
            }
            std::cmp::Ordering::Greater => {
                continue;
            }
        }
    }

    let mut tiles = BTreeSet::new();

    let s = *prev
        .keys()
        .filter(|el| el.position == *end.0)
        .min_by(|el1, el2| distances[el1].cmp(&distances[el2]))
        .unwrap();

    let mut q = VecDeque::from([s]);

    while let Some(u) = q.pop_front() {
        tiles.insert(u);
        if u.position == *start.0 {
            continue;
        }
        for tile in &prev[&u] {
            if !tiles.contains(tile) {
                q.push_front(*tile);
            }
        }
    }

    let tiles: Vec<(usize, usize)> = tiles.into_iter().map(|el| el.position).unique().collect();

    // // // dbg!(seq);
    // for r in 0..height {
    //     for c in 0..width {
    //         let ch = match tiles.contains(&(r, c)) {
    //             true => 'O',
    //             false => input[&(r, c)],
    //         };
    //         eprint!("{}", ch);
    //     }
    //     eprintln!()
    // }

    // todo!();

    Some(tiles.len() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
