use std::fs;

const FILE_PATH: &str = "11.txt";
const DIM: usize = 2;

fn main() {
    let galaxies = load_galaxies();
    for expansion in [2, 1_000_000] {
        let galaxies = expand_galaxies(galaxies.clone(), expansion);
        println!("{}", compute_distances(&galaxies));
    }
}

fn load_galaxies() -> Vec<[usize; DIM]> {
    let mut result: Vec<[usize; DIM]> = Vec::new();
    let input = fs::read_to_string(FILE_PATH).unwrap();
    for (y, line) in input.lines().enumerate() {
        for (x, value) in line.chars().enumerate() {
            if value == '#' {
                result.push([x, y]);
            }
        }
    }
    result
}

fn expand_galaxies(mut input: Vec<[usize; DIM]>, expansion: usize) -> Vec<[usize; DIM]> {
    for axis in 0..DIM {
        let mut result: Vec<[usize; DIM]> = Vec::new();
        let largest = input.iter().map(|p| p[axis]).max().unwrap();
        let mut shift: Vec<usize> = vec![expansion - 1; largest + 1];
        for point in &input {
            shift[point[axis]] = 0;
        }
        let mut cumulative: usize = 0;
        for value in &mut shift {
            cumulative += *value;
            *value = cumulative;
        }
        for mut point in input {
            point[axis] += shift[point[axis]];
            result.push(point);
        }
        input = result;
    }
    input
}

fn compute_distances(input: &Vec<[usize; DIM]>) -> i64 {
    let mut result: i64 = 0;
    for (index, p1) in input.iter().enumerate() {
        for p2 in input.iter().skip(index + 1) {
            for axis in 0..DIM {
                let v1: i64 = p1[axis].try_into().unwrap();
                let v2: i64 = p2[axis].try_into().unwrap();
                result += (v1 - v2).abs()
            }
        }
    }
    result
}
