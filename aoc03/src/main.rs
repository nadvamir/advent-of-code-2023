use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Default, Clone, Copy)]
struct Number {
    val: i32,
    is_part: bool,
}

struct Schematic {
    numbers: Vec<Number>,
    grid: Vec<Vec<usize>>, // mapping from the coordinates to the index in numbers
}

impl Schematic {
    fn new(lines: &[String]) -> Schematic {
        let mut schematic = Schematic {
            numbers: vec![Default::default()],
            grid: vec![vec![0; lines[0].len()]; lines.len()],
        };
        schematic.parse_input(lines);
        schematic
    }

    fn parse_input(&mut self, lines: &[String]) {
        for (row, line) in lines.iter().enumerate() {
            self.parse_line(row, line);
        }
        for (row, line) in lines.iter().enumerate() {
            self.mark_parts_in_line(row, line);
        }
    }

    fn parse_line(&mut self, row: usize, line: &str) {
        let mut curr_num: Number = Default::default();

        // fill out numbers
        for (col, c) in line.chars().enumerate() {
            if c.is_numeric() {
                let n = c as i32 - '0' as i32;
                curr_num.val = curr_num.val * 10 + n;
                self.grid[row][col] = self.numbers.len();
            } else {
                curr_num = self.record_new_num(curr_num);
            }
        }
        self.record_new_num(curr_num);
    }

    fn mark_parts_in_line(&mut self, row: usize, line: &str) {
        for (col, c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                self.mark_parts(row, col);
            }
        }
    }

    fn record_new_num(&mut self, num: Number) -> Number {
        if num.val != 0 {
            self.numbers.push(num);
            Default::default()
        } else {
            num
        }
    }

    fn mark_parts(&mut self, row: usize, col: usize) {
        let offsets = [-1, 0, 1];

        for &di in &offsets {
            for &dj in &offsets {
                if let (Some(i), Some(j)) = (row.checked_add_signed(di), col.checked_add_signed(dj)) {
                    if i < self.grid.len() && j < self.grid[i].len() {
                        self.numbers[self.grid[i][j]].is_part = true;
                    }
                }
            }
        }
    }
}

fn solve(lines: &[String]) -> i32 {
    let schematic = Schematic::new(lines);

    schematic
        .numbers
        .iter()
        .filter(|n| n.is_part)
        .map(|n| n.val)
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
    fn test_parsing_numbers() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let schematic = Schematic::new(&lines);
        let nums: i32 = schematic.numbers.iter().map(|n| n.val).sum();
        assert_eq!(nums, 4533);
    }

    #[test]
    fn test_solution1() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_solution2() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
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
