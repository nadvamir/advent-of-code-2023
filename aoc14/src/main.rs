use core::num;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn tilt_row(canvas: &mut Vec<String>, j: usize, i: usize) -> usize {
    if i < canvas.len() && canvas[i].as_bytes()[j] == b'#' {
        return i + 1;
    }
    let mut row = i;
    let mut num_stones = 0;

    while row < canvas.len() {
        let item = canvas[row].as_bytes()[j];
        if item == b'O' {
            num_stones += 1;
        }
        else if item == b'#' {
            break;
        }
        row += 1;
    }

    let mut stone_row = i;
    while num_stones > 0 {
        unsafe {
            canvas[stone_row].as_bytes_mut()[j] = b'O';
        }
        num_stones -= 1;
        stone_row += 1;
    }

    while stone_row < row {
        unsafe {
            canvas[stone_row].as_bytes_mut()[j] = b'.';
        }
        stone_row += 1;
    }

    return row;
}

fn tilt(lines: &[String]) -> Vec<String> {
    let mut canvas: Vec<String> = lines.iter().map(|s| (*s).clone()).collect();
    for j in 0..lines[0].len() {
        let mut i = 0;
        while i < canvas.len() {
            i = tilt_row(&mut canvas, j, i);
        }
    }
    canvas
}

fn solve(lines: &[String]) -> usize {
    let tilted = tilt(lines);
    tilted
        .iter()
        .enumerate()
        .map(|(i, line)| line.chars().filter(|c| *c == 'O').count() * (lines.len() - i))
        .sum()
}

// ----------------------------------------------------------------------------
fn solve2(lines: &[String]) -> usize {
    0
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 136);
    }

    #[test]
    fn test_solution2() {
        let input = r"
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
