use std::{collections::BTreeSet, fmt::Display};

use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Tile {
    Wall,
    Box,
    Robot,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            '@' => Tile::Robot,
            'O' => Tile::Box,
            _ => unimplemented!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match &self {
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::Robot => '@',
            Tile::Empty => '.',
        };

        write!(f, "{ch}")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn get_coord(&self) -> (isize, isize) {
        match &self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    const fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

const fn add_coords((a_r, a_c): (isize, isize), (b_r, b_c): (isize, isize)) -> (isize, isize) {
    (a_r + b_r, a_c + b_c)
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unimplemented!(),
        }
    }
}

struct Grid {
    elements: Vec<Vec<Tile>>,
    robot_location: (isize, isize),
}

impl Grid {
    fn print_grid(&self) {
        eprintln!("robots position {:?}", self.robot_location);
        for row in &self.elements {
            for col in row {
                eprint!("{col}");
            }
            eprintln!();
        }
    }

    fn get_at(&self, (row, col): (isize, isize)) -> Tile {
        self.elements[row as usize][col as usize]
    }

    fn update_at(&mut self, (row, col): (isize, isize), value: Tile) {
        self.elements[row as usize][col as usize] = value
    }
}

fn parse(input: &str) -> (Grid, Vec<Direction>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let directions: Vec<Direction> = parts[1]
        .replace("\n", "")
        .chars()
        .map(Direction::from)
        .collect();

    let grid: Vec<Vec<Tile>> = parts[0]
        .split("\n")
        .map(|row| row.chars().map(Tile::from).collect())
        .collect();

    let robot_location = grid
        .iter()
        .enumerate()
        .find_map(|(row_id, r)| {
            r.iter()
                .find_position(|el| **el == Tile::Robot)
                .map(|(col_id, _)| (row_id as isize, col_id as isize))
        })
        .unwrap();

    (
        Grid {
            elements: grid,
            robot_location,
        },
        directions,
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut grid, dirs) = parse(input);
    // grid.print_grid();

    for dir in dirs {
        move_robot_in_dir(&mut grid, dir);
    }
    // grid.print_grid();
    Some(calc_gps_coords(&grid))
}

fn move_robot_in_dir(grid: &mut Grid, dir: Direction) {
    let mut no_wall = true;
    let mut has_space = false;
    let mut cur_pos = grid.robot_location;

    while no_wall && !has_space {
        cur_pos = add_coords(cur_pos, dir.get_coord());
        let tile = grid.get_at(cur_pos);
        match tile {
            Tile::Wall => no_wall = false,
            Tile::Box => continue,
            Tile::Empty => has_space = true,
            Tile::Robot => panic!("The robot should not be seen here"),
        }
    }

    match (has_space, no_wall) {
        (true, true) => {
            let mut robot_reached = false;

            while !robot_reached {
                let prev_pos = add_coords(cur_pos, dir.opposite().get_coord());
                let tile = grid.get_at(prev_pos);
                grid.update_at(cur_pos, tile);
                if tile == Tile::Robot {
                    robot_reached = true;
                    grid.robot_location = cur_pos;
                }

                cur_pos = prev_pos;
            }

            // the robot moved off of the tile
            grid.update_at(cur_pos, Tile::Empty);
        }
        (false, false) => (),
        _ => unimplemented!(),
    }
}

fn calc_gps_coords(grid: &Grid) -> u64 {
    grid.elements
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, tile)| {
                if *tile == Tile::Box {
                    return Some(100 * row_idx as u64 + col_idx as u64);
                }
                None
            })
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FatTile {
    Robot,
    Empty,
    Wall,
    RightBox,
    LeftBox,
}

impl Display for FatTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match &self {
            Self::Wall => '#',
            Self::LeftBox => '[',
            Self::RightBox => ']',
            Self::Robot => '@',
            Self::Empty => '.',
        };

        write!(f, "{ch}")
    }
}

struct FatGrid {
    elements: Vec<Vec<FatTile>>,
    robot_position: (isize, isize),
}

impl From<Vec<Vec<Tile>>> for FatGrid {
    fn from(value: Vec<Vec<Tile>>) -> Self {
        let elements: Vec<Vec<FatTile>> = value
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(|tile| match tile {
                        Tile::Box => [FatTile::LeftBox, FatTile::RightBox],
                        Tile::Empty => [FatTile::Empty; 2],
                        Tile::Wall => [FatTile::Wall; 2],
                        Tile::Robot => [FatTile::Robot, FatTile::Empty],
                    })
                    .collect()
            })
            .collect();
        let robot_position = elements
            .iter()
            .enumerate()
            .find_map(|(row_id, r)| {
                r.iter()
                    .find_position(|el| **el == FatTile::Robot)
                    .map(|(col_id, _)| (row_id as isize, col_id as isize))
            })
            .unwrap();
        Self {
            elements,
            robot_position,
        }
    }
}

impl FatGrid {
    fn print_grid(&self) {
        for row in &self.elements {
            for col in row {
                eprint!("{col}");
            }
            eprintln!();
        }
    }

    fn get_at(&self, (row, col): (isize, isize)) -> FatTile {
        self.elements[row as usize][col as usize]
    }

    fn update_at(&mut self, (row, col): (isize, isize), value: FatTile) {
        self.elements[row as usize][col as usize] = value
    }

    fn get_affected_tiles(&self, dir: Direction) -> CanGo {
        // let tile = self.get_at(pos);
        let mut targets = BTreeSet::new();
        let mut to_visit = BTreeSet::new();
        to_visit.insert(self.robot_position);

        while !to_visit.is_empty() {
            let pos = to_visit.pop_first().unwrap();
            if targets.contains(&pos) {
                continue;
            }
            targets.insert(pos);

            let new_pos = add_coords(pos, dir.get_coord());
            let tile = self.get_at(new_pos);

            match tile {
                FatTile::Robot => unreachable!(),
                FatTile::Empty => (),
                FatTile::Wall => return CanGo::NoPush,
                FatTile::RightBox => {
                    to_visit.insert(add_coords(new_pos, Direction::Left.get_coord()));
                    to_visit.insert(new_pos);
                }
                FatTile::LeftBox => {
                    to_visit.insert(add_coords(new_pos, Direction::Right.get_coord()));
                    to_visit.insert(new_pos);
                }
            }
        }
        // targets

        CanGo::Push(
            targets
                .into_iter()
                .map(|pos| (pos, self.get_at(pos)))
                .collect(),
        )
    }

    fn move_to(&mut self, dir: Direction) {
        match self.get_affected_tiles(dir) {
            CanGo::Push(btree_set) => {
                btree_set
                    .iter()
                    .for_each(|(pos, _)| self.update_at(*pos, FatTile::Empty));

                btree_set.iter().for_each(|(pos, tile)| {
                    let new_pos = add_coords(*pos, dir.get_coord());
                    if *tile == FatTile::Robot {
                        self.robot_position = new_pos;
                    }
                    self.update_at(new_pos, *tile);
                });
            }
            CanGo::NoPush => (),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum CanGo {
    Push(BTreeSet<((isize, isize), FatTile)>),
    NoPush,
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, dirs) = parse(input);
    let mut grid = FatGrid::from(grid.elements);

    // grid.print_grid();

    for dir in dirs {
        grid.move_to(dir);
    }

    // grid.print_grid();

    Some(
        grid.elements
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(col_idx, tile)| (row_idx, col_idx, tile))
            })
            .filter_map(|(row_idx, col_idx, tile)| match tile {
                FatTile::LeftBox => Some(100 * row_idx as u64 + col_idx as u64),
                _ => None,
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
