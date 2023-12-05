use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Default)]
struct Card {
    id: i32,
    winning: HashSet<i32>,
    yours: HashSet<i32>,
}

fn parse_card(line: &str) -> Card {
    if let (Some(col_idx), Some(pipe_idx)) = (line.find(':'), line.find('|')) {
        let id: i32 = line[0..col_idx].split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .expect("Id has to be a number");
        let winning: HashSet<i32> = line[col_idx + 1..pipe_idx]
            .split_whitespace()
            .map(|n| n.parse().expect("Expect numbers"))
            .collect();
        let yours: HashSet<i32> = line[pipe_idx + 1..]
            .split_whitespace()
            .map(|n| n.parse().expect("Expect numbers"))
            .collect();
        winning.intersection(&yours).collect::<Vec<&i32>>().len();
        Card { id, winning, yours }
    } else {
        Default::default()
    }
}

fn solve(lines: &[String]) -> i32 {
    lines
        .iter()
        .map(|s| parse_card(s))
        .map(|c| {
            c.winning
                .intersection(&c.yours)
                .collect::<Vec<&i32>>()
                .len() as i32
        })
        .filter(|matches| *matches > 0)
        .map(|matches| 1 << (matches - 1))
        .sum()
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
    fn test_parse_card() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_card(input);
        assert_eq!(
            card,
            Card {
                id: 1,
                winning: [41, 48, 83, 86, 17].iter().cloned().collect(),
                yours: [83, 86, 6, 31, 17, 9, 48, 53].iter().cloned().collect(),
            }
        );
    }

    #[test]
    fn test_solution1() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_solution2() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
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
