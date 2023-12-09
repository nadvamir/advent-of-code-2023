use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn stoi(s: &str) -> i32 {
    s.parse::<i32>().unwrap_or_default()
}

fn parse_races(lines: &[String]) -> Vec<(i32, i32)> {
    lines[0][10..].split_whitespace().map(stoi).zip(lines[1][10..].split_whitespace().map(stoi)).collect()
}

fn solve_eq(time: f64, dist: f64) -> (f64, f64) {
    let discr = (time * time - 4.0 * dist).sqrt();
    ((-time + discr) / -2.0, (-time - discr) / -2.0)
}

fn calc_n_ways(x0: f64, x1: f64) -> i32 {
    (x1.ceil() - x0.floor() - 1.0) as i32
}

fn solve(lines: &[String]) -> i32 {
    parse_races(lines)
        .iter()
        .map(|(t, d)| solve_eq(*t as f64, *d as f64))
        .map(|(x0, x1)| calc_n_ways(x0, x1))
        .reduce(|acc, n| acc * n)
        .unwrap_or_default()
}

// ----------------------------------------------------------------------------
fn solve2(lines: &[String]) -> i32 {
    0
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calc_n_ways() {
        assert_eq!(calc_n_ways(1.7, 5.3), 4);
        assert_eq!(calc_n_ways(3.4, 11.5), 8);
        assert_eq!(calc_n_ways(10.0, 20.0), 9);
    }

    #[test]
    fn test_solution1() {
        let input = r"Time:      7  15   30
Distance:  9  40  200
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_solution2() {
        let input = r"Time:      7  15   30
Distance:  9  40  200
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 0);
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
