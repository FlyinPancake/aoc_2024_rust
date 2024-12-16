use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    position: (usize, usize),
    direction: Direction,
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

    distances
        .entry(Node {
            position: *start.0,
            direction: Direction::East,
        })
        .insert_entry(0);

    distances
        .entry(Node {
            position: *start.0,
            direction: Direction::North,
        })
        .insert_entry(1000);
    distances
        .entry(Node {
            position: *start.0,
            direction: Direction::West,
        })
        .insert_entry(2000);
    distances
        .entry(Node {
            position: *start.0,
            direction: Direction::South,
        })
        .insert_entry(1000);

    for (k, v) in input.iter() {
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            if *v == 'S' {
                continue;
            }

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

    let mut tc = None;

    'outer: while let Some(coords) = queue.pop() {
        for (neighbor_pos, dir) in neighbors(coords.node) {
            let neighbor = Node {
                position: neighbor_pos,
                direction: dir,
            };
            let cost = match input[&neighbor_pos] {
                '#' => continue,
                'E' => {
                    tc = Some(coords.cost + 1);
                    prev.insert(neighbor, coords.node);
                    break 'outer;
                }
                _ => distances[&coords.node] + 1 + dir.turn_cost(&coords.node.direction),
            };

            for dir in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                let neighbor_turned = Node {
                    position: neighbor_pos,
                    direction: dir,
                };
                let cost = cost + dir.turn_cost(&neighbor.direction);
                if cost < distances[&neighbor_turned] {
                    distances.entry(neighbor_turned).insert_entry(cost);
                    prev.entry(neighbor_turned).insert_entry(coords.node);
                    queue.push(State {
                        cost,
                        node: neighbor_turned,
                    });
                }
            }
        }
    }

    // let mut seq = VecDeque::new();

    // let mut u = prev
    //     .keys()
    //     .find(|Node { position, .. }| *position == *end.0)
    //     .unwrap()
    //     .to_owned();

    // while u.position != *start.0 {
    //     seq.push_front(u);
    //     u = prev[&u]
    // }

    // // dbg!(seq);
    // for r in 0..height {
    //     for c in 0..width {
    //         let ch = match seq.iter().find(|el| el.position == (r, c)) {
    //             Some(Node { direction, .. }) => match direction {
    //                 Direction::North => '^',
    //                 Direction::West => '<',
    //                 Direction::South => 'v',
    //                 Direction::East => '>',
    //             },
    //             None => input[&(r, c)],
    //         };
    //         eprint!("{}", ch);
    //     }
    //     eprintln!()
    // }

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

    distances
        .entry(Node {
            position: *start.0,
            direction: Direction::East,
        })
        .insert_entry(0);

    distances
        .entry(Node {
            position: *start.0,
            direction: Direction::North,
        })
        .insert_entry(1000);
    distances
        .entry(Node {
            position: *start.0,
            direction: Direction::West,
        })
        .insert_entry(2000);
    distances
        .entry(Node {
            position: *start.0,
            direction: Direction::South,
        })
        .insert_entry(1000);

    for (k, v) in input.iter() {
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            if *v == 'S' {
                continue;
            }

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

    let mut best_cost = i64::MAX;

    'outer: while let Some(coords) = queue.pop() {
        for (neighbor_pos, dir) in neighbors(coords.node) {
            let neighbor = Node {
                position: neighbor_pos,
                direction: dir,
            };
            let cost = match input[&neighbor_pos] {
                '#' => continue,

                'E' => {
                    match (coords.cost + 1).cmp(&best_cost) {
                        std::cmp::Ordering::Less => {
                            best_cost = coords.cost + 1;
                            prev.entry(neighbor_pos).insert_entry(vec![coords.node]);
                        }
                        std::cmp::Ordering::Equal => {
                            prev.entry(neighbor_pos)
                                .and_modify(|el| el.push(coords.node));
                        }
                        std::cmp::Ordering::Greater => break 'outer,
                    }
                    continue;
                }

                _ => distances[&coords.node] + 1 + dir.turn_cost(&coords.node.direction),
            };

            for dir in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                let neighbor_turned = Node {
                    position: neighbor_pos,
                    direction: dir,
                };
                let cost = cost + dir.turn_cost(&neighbor.direction);

                match cost.cmp(&distances[&neighbor_turned]) {
                    std::cmp::Ordering::Less => {
                        distances.entry(neighbor_turned).insert_entry(cost);
                        prev.entry(neighbor_turned.position)
                            .insert_entry(vec![coords.node]);
                        queue.push(State {
                            cost,
                            node: neighbor_turned,
                        });
                    }
                    std::cmp::Ordering::Equal => {
                        prev.entry(neighbor_turned.position)
                            .and_modify(|v| v.push(coords.node));
                    }
                    std::cmp::Ordering::Greater => {
                        continue;
                    }
                }
            }
        }
    }

    let mut tiles = BTreeSet::new();
    let mut q = VecDeque::new();

    let u = prev
        .keys()
        .find(|position| **position == *end.0)
        .unwrap()
        .to_owned();

    q.push_back(u);

    while let Some(u) = q.pop_front() {
        tiles.insert(u);
        if u == *start.0 {
            continue;
        }
        for tile in &prev[&u] {
            if !tiles.contains(&tile.position) {
                q.push_front(tile.position);
            }
        }
    }

    // // dbg!(seq);
    for r in 0..height {
        for c in 0..width {
            let ch = match tiles.contains(&(r, c)) {
                true => 'O',
                false => input[&(r, c)],
            };
            eprint!("{}", ch);
        }
        eprintln!()
    }
    todo!();
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
