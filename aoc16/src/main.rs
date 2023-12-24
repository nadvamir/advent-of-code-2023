use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn get_next((i, j): (i32, i32), dir: Direction) -> ((i32, i32), Direction) {
    match dir {
        Direction::N => ((i - 1, j), dir),
        Direction::E => ((i, j + 1), dir),
        Direction::S => ((i + 1, j), dir),
        Direction::W => ((i, j - 1), dir),
    }
}

fn detect_energised(grid: &[String], init: ((i32, i32), Direction)) -> HashSet<(usize, usize)> {
    let mut energised: HashSet<(usize, usize)> = Default::default();
    let mut visited: HashSet<(i32, i32, Direction)> = Default::default();
    let mut q = VecDeque::new();
    q.push_back(init);

    while !q.is_empty() {
        let ((i, j), dir) = q.pop_front().unwrap();

        if i < 0 || j < 0 || i as usize == grid.len() || j as usize == grid[0].len() {
            continue;
        }
        if visited.contains(&(i, j, dir)) {
            continue;
        }

        energised.insert((i as usize, j as usize));
        visited.insert((i, j, dir));

        match grid[i as usize].as_bytes()[j as usize] {
            b'-' => {
                if dir == Direction::E || dir == Direction::W {
                    q.push_back(get_next((i, j), dir));
                } else {
                    q.push_back(get_next((i, j), Direction::E));
                    q.push_back(get_next((i, j), Direction::W));
                }
            }
            b'|' => {
                if dir == Direction::N || dir == Direction::S {
                    q.push_back(get_next((i, j), dir));
                } else {
                    q.push_back(get_next((i, j), Direction::N));
                    q.push_back(get_next((i, j), Direction::S));
                }
            }
            b'/' => {
                let dir = match dir {
                    Direction::N => Direction::E,
                    Direction::E => Direction::N,
                    Direction::S => Direction::W,
                    Direction::W => Direction::S,
                };
                q.push_back(get_next((i, j), dir));
            }
            b'\\' => {
                let dir = match dir {
                    Direction::N => Direction::W,
                    Direction::W => Direction::N,
                    Direction::S => Direction::E,
                    Direction::E => Direction::S,
                };
                q.push_back(get_next((i, j), dir));
            }
            _ => q.push_back(get_next((i, j), dir)),
        }
    }

    energised
}

fn solve(lines: &[String]) -> usize {
    let energised = detect_energised(lines, ((0, 0), Direction::E));
    energised.len()
}

// ----------------------------------------------------------------------------
fn solve2(lines: &[String]) -> usize {
    let mut max_energised = 0;
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if i == 0 {
                max_energised = std::cmp::max(
                    max_energised,
                    detect_energised(lines, ((i as i32, j as i32), Direction::S)).len(),
                );
            }
            if j == 0 {
                max_energised = std::cmp::max(
                    max_energised,
                    detect_energised(lines, ((i as i32, j as i32), Direction::E)).len(),
                );
            }
            if i == lines.len() - 1 {
                max_energised = std::cmp::max(
                    max_energised,
                    detect_energised(lines, ((i as i32, j as i32), Direction::N)).len(),
                );
            }
            if j == lines[0].len() - 1 {
                max_energised = std::cmp::max(
                    max_energised,
                    detect_energised(lines, ((i as i32, j as i32), Direction::W)).len(),
                );
            }
        }
    }
    max_energised
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 46);
    }

    #[test]
    fn test_solution2() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 51);
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
