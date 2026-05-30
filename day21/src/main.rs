use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "sample.txt";
const ITERATIONS: usize = 50;

#[derive(Debug)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    GardenPlot,
    Rock,
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).unwrap();
    let (grid, starting_position) = parse_grid(&input);
    // pt1(&grid, starting_position);
    // pt2(&grid, starting_position, 6);
    // pt2(&grid, starting_position, 10);
    // pt2(&grid, starting_position, 50);
    pt2(&grid, starting_position, 100);
    // pt2(&grid, starting_position, 500);
    // pt2(&grid, starting_position, 1000);
    // pt2(&grid, starting_position, 5000);
}

fn pt1(grid: &Grid, starting_position: usize) {
    let mut possible_positions: HashSet<usize> = HashSet::new();
    possible_positions.insert(starting_position);
    for _ in 0..ITERATIONS {
        let mut next_possible_positions: HashSet<usize> = HashSet::new();
        for position in possible_positions {
            for_each_adjacent_position(position, grid.width, grid.height, |position| {
                match grid.cells[position] {
                    Cell::GardenPlot => _ = next_possible_positions.insert(position),
                    Cell::Rock => (),
                }
            })
        }
        possible_positions = next_possible_positions;
    }
    println!("{}", possible_positions.len());
}

fn for_each_adjacent_position<F>(position: usize, width: usize, height: usize, mut f: F) where F: FnMut(usize) {
    // Right
    if (position + 1) % width != 0 {
        f(position + 1);
    }
    // Left
    if position % width != 0 {
        f(position - 1);
    }
    // Up
    if position >= width {
        f(position - width);
    }
    // Down
    if position < (width * (height - 1)) {
        f(position + width);
    }
}

fn pt2(grid: &Grid, starting_position: usize, iterations: usize) {
    let scale = 1001;
    let repeated_width = grid.width * scale;
    let middle_position = (repeated_width + grid.width) * grid.height * (scale / 2);
    let starting_position = middle_position + starting_position;
    println!("{}", starting_position);
    let mut possible_positions: HashSet<usize> = HashSet::new();
    possible_positions.insert(starting_position);
    let mut grid_counts: Vec<usize> = vec![0; grid.width * grid.height];
    let mut grid_deltas: Vec<isize> = vec![0; grid.width * grid.height];
    for iteration in 0..iterations {
        let mut next_possible_positions: HashSet<usize> = HashSet::new();
        let mut next_grid_counts: Vec<usize> = vec![0; grid.width * grid.height];
        for position in possible_positions {
            for_each_adjacent_position_with_wrapping(position, repeated_width, |position| {
                let index = (position / repeated_width) % grid.height * grid.width + position % grid.width;
                match grid.cells[index] {
                    Cell::GardenPlot => {
                        if next_possible_positions.insert(position) {
                            next_grid_counts[index] += 1;
                        }
                    },
                    Cell::Rock => (),
                }
            })
        }
        let mut next_grid_deltas: Vec<isize> = Vec::new();
        for (&old, &new) in std::iter::zip(&grid_counts, &next_grid_counts) {
            next_grid_deltas.push(new as isize - old as isize);
        }
        let mut next_grid_accels: Vec<isize> = Vec::new();
        for (&old, &new) in std::iter::zip(&grid_deltas, &next_grid_deltas) {
            next_grid_accels.push(new - old);
        }
        if iteration % 11 == 3 {
            // println!("{:4} {:4} {:4}", next_grid_deltas[21], next_grid_deltas[40], next_grid_deltas[60]);
            for &value in &next_grid_counts[..20] {
                print!("{:4}", value);
            }
            println!();
        }
        possible_positions = next_possible_positions;
        grid_counts = next_grid_counts;
        grid_deltas = next_grid_deltas;
    }
    println!("{}", possible_positions.len());
}

fn for_each_adjacent_position_with_wrapping<F>(position: usize, width: usize, mut f: F) where F: FnMut(usize) {
    assert!(position >= width);
    assert_ne!(position % width, 0);
    assert_ne!((position + 1) % width, 0);
    f(position + 1);
    f(position - 1);
    f(position - width);
    f(position + width);
}

fn parse_grid(input: &str) -> (Grid, usize) {
    let mut cells: Vec<Cell> = Vec::new();
    let mut width: Option<usize> = None;
    let mut starting_position: Option<usize> = None;
    for (row, line) in input.lines().enumerate() {
        match width {
            None => width = Some(line.len()),
            Some(width) => assert_eq!(width, line.len()),
        }
        for (col, value) in line.chars().enumerate() {
            cells.push(match value {
                'S' => {
                    assert_eq!(starting_position, None);
                    starting_position = Some(row * width.unwrap() + col);
                    Cell::GardenPlot
                },
                '.' => Cell::GardenPlot,
                '#' => Cell::Rock,
                _ => unreachable!(),
            });
        }
    }
    let width = width.unwrap();
    let height = cells.len() / width;
    return (Grid { cells, width, height }, starting_position.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_each_adjacent_position_with_wrapping() {
        assert_eq!(adjacent_positions_with_wrapping(20, 5), vec![]);
    }

    fn adjacent_positions_with_wrapping(position: usize, width: usize) -> Vec<usize> {
        let mut positions: Vec<usize> = Vec::new();
        for_each_adjacent_position_with_wrapping(position, width, |position| positions.push(position));
        return positions;
    }
}