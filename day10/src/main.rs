use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

const DIRECTIONS: &[Direction] = &[Direction::North, Direction::South, Direction::East, Direction::West];

impl Direction {
    fn invert(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

fn main() {
    let input = fs::read_to_string("10.txt").unwrap();
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    // Find the initial position.
    let initial_position = find_initial_position(&grid).unwrap();

    let result = search(initial_position, &grid).unwrap();
    println!("{}", result/2);
}

fn find_initial_position(grid: &Vec<&[u8]>) -> Option<Position> {
    for (y, &cols) in grid.iter().enumerate() {
        for (x, &value) in cols.iter().enumerate() {
            if value == b'S' {
                return Some(Position { x, y });
            }
        }
    }
    None
}

fn search(initial_position: Position, grid: &Vec<&[u8]>) -> Option<u32> {
    for &initial_direction in DIRECTIONS {
        let result =
            search_from_initial_position(initial_position, initial_direction, &grid);
        match result {
            Some(value) => return Some(value),
            None => continue,
        }
    }
    None
}

fn search_from_initial_position(
    initial_position: Position,
    initial_direction: Direction,
    grid: &Vec<&[u8]>,
) -> Option<u32> {
    // Grab the first position we should visit.
    let mut search_positions: VecDeque<(Position, u32)> = VecDeque::new();
    match move_from(initial_position, initial_direction, None, grid) {
        None => return None,
        Some(position) => search_positions.push_back((position, 1)),
    }
    // Track which positions we've already visited.
    let mut checked_positions: HashSet<Position> = HashSet::new();
    loop {
        let (position, distance) = match search_positions.pop_front() {
            None => return None,
            Some((position, distance)) => (position, distance),
        };
        // If we've already checked this position, don't check it again.
        if !checked_positions.insert(position) {
            continue;
        }
        if position == initial_position {
            return Some(distance);
        }
        for &direction in DIRECTIONS {
            match move_from(position, direction, Some(initial_direction), grid) {
                None => continue,
                Some(position) => search_positions.push_back((position, distance + 1)),
            }
        }
    }
}

fn move_from(
    src: Position,
    dir: Direction,
    blocked_edge: Option<Direction>,
    grid: &Vec<&[u8]>,
) -> Option<Position> {
    // Can we leave `src` in `dir` based on the symbol it contains?
    if !can_leave(grid[src.y][src.x], dir, blocked_edge) {
        return None;
    }
    // Can we leave `src` in `dir` and stay within the grid?
    let dst: Position = match dir {
        Direction::North => {
            if src.y == 0 {
                return None;
            } else {
                Position {
                    x: src.x,
                    y: src.y - 1,
                }
            }
        }
        Direction::South => {
            if src.y + 1 >= grid.len() {
                return None;
            } else {
                Position {
                    x: src.x,
                    y: src.y + 1,
                }
            }
        }
        Direction::East => {
            if src.x + 1 >= grid[0].len() {
                return None;
            } else {
                Position {
                    x: src.x + 1,
                    y: src.y,
                }
            }
        }
        Direction::West => {
            if src.x == 0 {
                return None;
            } else {
                Position {
                    x: src.x - 1,
                    y: src.y,
                }
            }
        }
    };
    // Can we enter `dst` from `dir` based on the symbol it contains?
    if !can_leave(grid[dst.y][dst.x], dir.invert(), blocked_edge) {
        return None;
    }
    Some(dst)
}

fn can_leave(symbol: u8, direction: Direction, blocked_edge: Option<Direction>) -> bool {
    match symbol {
        b'S' => match blocked_edge {
            None => true,
            Some(blocked_edge) => direction != blocked_edge,
        },
        b'|' => matches!(direction, Direction::North | Direction::South),
        b'F' => matches!(direction, Direction::South | Direction::East),
        b'7' => matches!(direction, Direction::South | Direction::West),
        b'L' => matches!(direction, Direction::North | Direction::East),
        b'J' => matches!(direction, Direction::North | Direction::West),
        b'-' => matches!(direction, Direction::East | Direction::West),
        b'.' => false,
        _ => unreachable!(),
    }
}
