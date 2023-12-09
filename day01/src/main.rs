use std::fs;

fn main() {
    let contents = fs::read_to_string("../01.txt").unwrap();
    println!("{}", pt1(&contents));
    println!("{}", pt2(&contents));
}

fn pt1(contents: &str) -> u32 {
    let mut result = 0;
    for line in contents.lines() {
        let mut iter = line.chars().filter_map(|ch| ch.to_digit(10)).peekable();
        let c1 = iter.peek().unwrap().clone();
        let c2 = iter.last().unwrap().clone();
        result += 10 * c1 + c2;
    }
    result
}

fn pt2(contents: &str) -> u32 {
    let mut result = 0;
    for line in contents.lines() {
        let mut iter = line
            .chars()
            .enumerate()
            .filter_map(|(pos, ch)| ch.to_digit(10).or_else(|| parse_text(&line[pos..])))
            .peekable();
        let c1 = iter.peek().unwrap().clone();
        let c2 = iter.last().unwrap().clone();
        result += 10 * c1 + c2;
    }
    result
}

fn parse_text(value: &str) -> Option<u32> {
    if value.starts_with("one") {
        Some(1)
    } else if value.starts_with("two") {
        Some(2)
    } else if value.starts_with("three") {
        Some(3)
    } else if value.starts_with("four") {
        Some(4)
    } else if value.starts_with("five") {
        Some(5)
    } else if value.starts_with("six") {
        Some(6)
    } else if value.starts_with("seven") {
        Some(7)
    } else if value.starts_with("eight") {
        Some(8)
    } else if value.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}
