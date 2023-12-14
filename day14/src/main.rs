use std::fs;
use std::iter;

const FILE_PATH: &str = "14.txt";

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

const DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

#[derive(Clone, Copy, Debug)]
enum Rock {
    Rolling,
    Immobile,
    Empty,
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).unwrap();
    {
        let mut platform = parse_platform(&input);
        tilt_platform(&mut platform, Direction::North);
        println!("{}", score_platform(&platform));
    }
    {
        let mut platform = parse_platform(&input);
        println!(
            "{}",
            tilt_platform_all_directions_repeatedly(&mut platform, 1_000_000_000)
        );
    }
}

fn parse_platform(input: &str) -> Vec<Vec<Rock>> {
    let mut platform: Vec<Vec<Rock>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<Rock> = Vec::new();
        for ch in line.chars() {
            row.push(match ch {
                '.' => Rock::Empty,
                '#' => Rock::Immobile,
                'O' => Rock::Rolling,
                _ => unreachable!(),
            });
        }
        platform.push(row);
    }
    platform
}

fn rows_iter(size: usize, direction: Direction) -> Box<dyn Iterator<Item = usize>> {
    match direction {
        Direction::South => Box::new((0..size).rev()),
        _ => Box::new(0..size),
    }
}

fn cols_iter(size: usize, direction: Direction) -> Box<dyn Iterator<Item = usize>> {
    match direction {
        Direction::East => Box::new((0..size).rev()),
        _ => Box::new(0..size),
    }
}

fn tilt_platform(platform: &mut Vec<Vec<Rock>>, direction: Direction) {
    for row in rows_iter(platform.len(), direction) {
        for col in cols_iter(platform[0].len(), direction) {
            if matches!(platform[row][col], Rock::Rolling) {
                roll_rock(platform, row, col, direction);
            }
        }
    }
}

fn roll_rock(platform: &mut Vec<Vec<Rock>>, row: usize, col: usize, direction: Direction) {
    let path: (
        Box<dyn Iterator<Item = usize>>,
        Box<dyn Iterator<Item = usize>>,
    ) = match direction {
        Direction::North => (Box::new((0..row).rev()), Box::new(iter::repeat(col))),
        Direction::South => (
            Box::new((row + 1)..platform.len()),
            Box::new(iter::repeat(col)),
        ),
        Direction::West => (Box::new(iter::repeat(row)), Box::new((0..col).rev())),
        Direction::East => (
            Box::new(iter::repeat(row)),
            Box::new((col + 1)..platform[0].len()),
        ),
    };
    let (mut old_row, mut old_col) = (row, col);
    for (new_row, new_col) in iter::zip(path.0, path.1) {
        if matches!(platform[new_row][new_col], Rock::Empty) {
            platform[new_row][new_col] = Rock::Rolling;
            platform[old_row][old_col] = Rock::Empty;
        } else {
            break;
        }
        (old_row, old_col) = (new_row, new_col);
    }
}

fn score_platform(platform: &Vec<Vec<Rock>>) -> usize {
    let mut result = 0;
    for (row, rocks) in platform.iter().enumerate() {
        for rock in rocks {
            if matches!(rock, Rock::Rolling) {
                result += platform.len() - row;
            }
        }
    }
    result
}

fn tilt_platform_all_directions_repeatedly(platform: &mut Vec<Vec<Rock>>, count: usize) -> usize {
    let mut scores: Vec<usize> = Vec::new();
    for _ in 0..count {
        tilt_platform_all_directions(platform);
        scores.push(score_platform(&platform));
        match find_cycle(&scores, 8) {
            None => continue,
            Some((base, cycle)) => {
                return cycle[(count - base - 1) % cycle.len()];
            }
        }
    }
    unreachable!();
}

fn tilt_platform_all_directions(platform: &mut Vec<Vec<Rock>>) {
    for &direction in DIRECTIONS {
        tilt_platform(platform, direction);
    }
}

fn find_cycle(scores: &[usize], cycle_count: usize) -> Option<(usize, &[usize])> {
    for cycle_length in 1..(scores.len() / cycle_count) {
        let cycled_scores = scores
            .iter()
            .rev()
            .take(cycle_length)
            .cycle()
            .take(cycle_count * cycle_length);
        let actual_scores = scores.iter().rev().take(cycle_count * cycle_length);
        if cycled_scores.eq(actual_scores) {
            return Some((
                scores.len() - cycle_length,
                &scores[scores.len() - cycle_length..],
            ));
        }
    }
    return None;
}
