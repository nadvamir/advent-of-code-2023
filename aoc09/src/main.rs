use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line(line: &String) -> Vec<i64> {
    line.split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect()
}

fn calculate_differences(nums: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut iter_diffs: Vec<Vec<i64>> = Default::default();
    iter_diffs.push(nums.clone());
    loop {
        let last = iter_diffs.last().unwrap();
        if last.len() == 1 {
            iter_diffs.push(vec![0]);
            break;
        }
        if last.iter().all(|n| *n == 0) {
            break;
        }
        let diffs: Vec<i64> = last[0..last.len() - 1]
            .iter()
            .zip(last[1..].iter())
            .map(|(a, b)| b - a)
            .collect();
        iter_diffs.push(diffs);
    }
    iter_diffs
}

fn get_next(nums: &Vec<i64>) -> i64 {
    let mut iter_diffs = calculate_differences(nums);
    for i in (1..iter_diffs.len()).rev() {
        let next = iter_diffs[i - 1].last().unwrap() + iter_diffs[i].last().unwrap();
        iter_diffs[i - 1].push(next);
    }
    *iter_diffs[0].last().unwrap()
}

fn solve(lines: &[String]) -> i64 {
    lines
        .iter()
        .map(parse_line)
        .map(|nums| get_next(&nums))
        .sum()
}

// ----------------------------------------------------------------------------
fn get_prev(nums: &Vec<i64>) -> i64 {
    let mut iter_diffs = calculate_differences(nums);
    for i in (1..iter_diffs.len()).rev() {
        let prev = iter_diffs[i - 1].first().unwrap() - iter_diffs[i].first().unwrap();
        iter_diffs[i - 1].insert(0, prev);
    }
    *iter_diffs[0].first().unwrap()
}

fn solve2(lines: &[String]) -> i64 {
    lines
        .iter()
        .map(parse_line)
        .map(|nums| get_prev(&nums))
        .sum()}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 114);
    }

    #[test]
    fn test_solution2() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 2);
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
