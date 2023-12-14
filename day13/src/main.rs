use nom;
use std::fs;
use std::iter;

const FILE_PATH: &str = "13.txt";

#[derive(Debug)]
struct Pattern {
    grid: Vec<Vec<char>>,
}

impl Pattern {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, grid) = nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::multi::many1(nom::character::complete::one_of(".#")),
        )(input)?;
        Ok((input, Self { grid }))
    }
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Pattern>> {
    nom::multi::separated_list1(
        nom::multi::count(nom::character::complete::newline, 2),
        Pattern::parse,
    )(input)
}

impl Pattern {
    fn find_line_with_mismatches(
        &self,
        count: usize,
        size: usize,
        is_width: bool,
        mismatch_goal: usize,
    ) -> Option<usize> {
        let mut mismatches: Vec<usize> = vec![0; size - 1];
        for index in 0..count {
            for line in 1..size {
                for (lhs, rhs) in iter::zip((0..line).rev(), line..size) {
                    let can_reflect = if is_width {
                        self.grid[index][lhs] == self.grid[index][rhs]
                    } else {
                        self.grid[lhs][index] == self.grid[rhs][index]
                    };
                    if !can_reflect {
                        mismatches[line - 1] += 1;
                    }
                }
            }
        }
        for (index, &mismatch_value) in mismatches.iter().enumerate() {
            if mismatch_value == mismatch_goal {
                return Some(index + 1);
            }
        }
        return None;
    }

    fn find_value_with_mismatches(&self, mismatch_goal: usize) -> usize {
        let (w, h) = (self.grid[0].len(), self.grid.len());
        if let Some(value) = self.find_line_with_mismatches(h, w, true, mismatch_goal) {
            return value;
        }
        if let Some(value) = self.find_line_with_mismatches(w, h, false, mismatch_goal) {
            return 100 * value;
        }
        unreachable!();
    }
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).unwrap();
    let (_, patterns) = parse(&input).unwrap();
    for mismatch_goal in [0, 1] {
        let mut result = 0;
        for pattern in &patterns {
            result += pattern.find_value_with_mismatches(mismatch_goal);
        }
        println!("{}", result);
    }
}
