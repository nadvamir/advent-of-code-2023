use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader.lines().collect()
}

fn find_first_digit<I>(iterator: I) -> char
where
    I: Iterator<Item = char>,
{
    for item in iterator {
        if item.is_numeric() {
            return item;
        }
    }
    return '0';
}

fn to_digit(c: char) -> i64 {
    c as i64 - '0' as i64
}

fn solve(lines: Vec<String>) -> i64 {
    let mut result: i64 = 0;
    for line in lines {
        let first = find_first_digit(line.chars());
        let last = find_first_digit(line.chars().rev());
        result += to_digit(first) * 10 + to_digit(last);
    }
    result
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let lines = input.lines().map(|line| line.to_string()).collect();
        let result = solve(lines);
        assert_eq!(result, 142);
    }
}

// ----------------------------------------------------------------------------
fn main() -> io::Result<()> {
    let filename = "src/input.in";
    let lines = read_lines(filename)?;

    println!("Answer: {}", solve(lines));

    Ok(())
}
