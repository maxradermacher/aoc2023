use std::cmp::max;
use std::collections::VecDeque;
use std::fs;

const FILE_PATH: &str = "16.txt";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_forward(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
        }
    }

    fn rotate_backward(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }

    fn bit_value(&self) -> u8 {
        match self {
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 4,
            Direction::Right => 8,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy, Debug)]
enum GridSpace {
    Empty,
    MirrorForwardSlash,
    MirrorBackwardSlash,
    HorizontalSplitter,
    VerticalSplitter,
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).unwrap();
    let (grid, width, height) = parse_grid(&input);
    // Part 1
    println!("{}", count_energized_tiles(&grid, width, height, Point { row: 0, col: 0 }, Direction::Right));
    // Part 2
    let mut result = 0;
    for row in 0..height {
        result = result.max(count_energized_tiles(&grid, width, height, Point { row, col: 0 }, Direction::Right));
        result = result.max(count_energized_tiles(&grid, width, height, Point { row, col: width - 1 }, Direction::Left));
    }
    for col in 0..width {
        result = result.max(count_energized_tiles(&grid, width, height, Point { row: 0, col }, Direction::Down));
        result = result.max(count_energized_tiles(&grid, width, height, Point { row: height - 1, col }, Direction::Up));
    }
    println!("{}", result);
}

fn parse_grid(input: &str) -> (Vec<GridSpace>, usize, usize) {
    let mut result: Vec<GridSpace> = Vec::new();
    let mut width: Option<usize> = None;
    let mut height: usize = 0;
    for line in input.lines() {
        for value in line.chars() {
            result.push(match value {
                '.' => GridSpace::Empty,
                '/' => GridSpace::MirrorForwardSlash,
                '\\' => GridSpace::MirrorBackwardSlash,
                '-' => GridSpace::HorizontalSplitter,
                '|' => GridSpace::VerticalSplitter,
                _ => unreachable!(),
            });
        }
        match width {
            None => width = Some(line.len()),
            Some(width) => assert!(width == line.len()),
        }
        height += 1;
    }
    (result, width.unwrap(), height)
}

fn count_energized_tiles(grid: &Vec<GridSpace>, width: usize, height: usize, position: Point, direction: Direction) -> usize {
    let mut beam = vec![0; width * height];

    let mut next: VecDeque<(Point, Direction)> = VecDeque::new();
    next.push_back((position, direction));

    while let Some((position, direction)) = next.pop_front() {
        let index = position.row * width + position.col;
        if beam[index] & direction.bit_value() != 0 {
            continue;
        }
        beam[index] |= direction.bit_value();
        for next_direction in next_directions(grid[index], direction) {
            let next_position = match next_direction {
                Direction::Up if position.row > 0 => Point { row: position.row - 1, ..position },
                Direction::Down if position.row < (height - 1) => Point { row: position.row + 1, ..position },
                Direction::Left if position.col > 0 => Point { col: position.col - 1, ..position },
                Direction::Right if position.col < (width - 1) => Point { col: position.col + 1, ..position },
                _ => continue,
            };
            next.push_back((next_position, next_direction));
        }
    }

    beam.iter().filter(|&v| *v != 0).count()
}

fn next_directions(grid_space: GridSpace, direction: Direction) -> Vec<Direction> {
    match grid_space {
        GridSpace::Empty => vec![direction],
        GridSpace::MirrorForwardSlash => vec![direction.rotate_forward()],
        GridSpace::MirrorBackwardSlash => vec![direction.rotate_backward()],
        GridSpace::HorizontalSplitter => match direction {
            Direction::Left | Direction::Right => vec![direction],
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
        }
        GridSpace::VerticalSplitter => match direction {
            Direction::Up | Direction::Down => vec![direction],
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
        }
    }
}
