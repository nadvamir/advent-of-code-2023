use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn find_first_digit<I>(iterator: I) -> char
where
    I: DoubleEndedIterator<Item = char>,
{
    for item in iterator {
        if item.is_numeric() {
            return item;
        }
    }
    return '0';
}

fn find_last_digit<I>(iter: I) -> char
where
    I: DoubleEndedIterator<Item = char>,
{
    find_first_digit(iter.rev())
}

fn to_digit(c: char) -> i64 {
    c as i64 - '0' as i64
}

fn solve(lines: &[String]) -> i64 {
    lines
        .iter()
        .map(|line| {
            let first = find_first_digit(line.chars());
            let last = find_last_digit(line.chars());
            to_digit(first) * 10 + to_digit(last)
        })
        .sum()
}

// ----------------------------------------------------------------------------
const MAPPING: [(&str, i64); 20] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn find_first_digit2(line: &String) -> i64 {
    MAPPING
        .iter()
        .map(|(k, v)| (line.find(k).unwrap_or(line.len()), *v))
        .min_by_key(|(idx, _)| *idx)
        .unwrap_or((0, 0))
        .1
}

fn find_last_digit2(line: &String) -> i64 {
    MAPPING
        .iter()
        .map(|(k, v)| match line.rfind(k) {
            Some(idx) => (idx as i64, *v),
            None => (-1, 0),
        })
        .max_by_key(|(idx, _)| *idx)
        .unwrap_or((0, 0))
        .1
}

fn solve2(lines: &[String]) -> i64 {
    lines
        .iter()
        .map(|line| find_first_digit2(line) * 10 + find_last_digit2(line))
        .sum()
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let input = r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_solution2() {
        let input = r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 281);
    }
}

// ----------------------------------------------------------------------------
fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader.lines().collect()
}

fn main() -> io::Result<()> {
    let filename = "src/input.in";
    let lines = read_lines(filename)?;

    println!("Answer, part 1: {}", solve(&lines));
    println!("Answer, part 2: {}", solve2(&lines));

    Ok(())
}
