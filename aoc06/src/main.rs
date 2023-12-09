use bigdecimal::FromPrimitive;
use bigdecimal::{BigDecimal, ToPrimitive};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn stoi(s: &str) -> BigDecimal {
    BigDecimal::from_str(s).unwrap()
}

fn parse_races(lines: &[String]) -> Vec<(BigDecimal, BigDecimal)> {
    lines[0][10..]
        .split_whitespace()
        .map(stoi)
        .zip(lines[1][10..].split_whitespace().map(stoi))
        .collect()
}

fn solve_eq(time: &BigDecimal, dist: &BigDecimal) -> (f64, f64) {
    let four = BigDecimal::from_i32(4).unwrap();
    let two = BigDecimal::from_i32(2).unwrap();

    let discr = (time * time - dist * &four).sqrt().unwrap();
    let (x0, x1) = ((time - &discr) / &two, (time + &discr) / &two);
    (x0.to_f64().unwrap(), x1.to_f64().unwrap())
}

fn calc_n_ways(x0: f64, x1: f64) -> i64 {
    (x1.ceil() - x0.floor() - 1.0) as i64
}

fn solve(lines: &[String]) -> i64 {
    parse_races(lines)
        .iter()
        .map(|(t, d)| solve_eq(t, d))
        .map(|(x0, x1)| calc_n_ways(x0, x1))
        .reduce(|acc, n| acc * n)
        .unwrap_or_default()
}

fn parse_races2(lines: &[String]) -> (BigDecimal, BigDecimal) {
    let remove_whitespace = |s: &str| s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    (
        BigDecimal::from_str(&remove_whitespace(&lines[0][10..])).unwrap(),
        BigDecimal::from_str(&remove_whitespace(&lines[1][10..])).unwrap(),
    )
}

// ----------------------------------------------------------------------------
fn solve2(lines: &[String]) -> i64 {
    let (t, d) = parse_races2(lines);
    let (x0, x1) = solve_eq(&t, &d);
    println!("{} {} -> {}, {}", &t, &d, x0, x1);
    calc_n_ways(x0, x1)
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
        assert_eq!(result, 71503);
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
