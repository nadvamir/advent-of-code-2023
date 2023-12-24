use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn directions(dir: Direction, momentum: usize) -> Vec<(Direction, usize)> {
    let mut dirs = vec![];

    if momentum > 0 {
        dirs.push((dir, momentum - 1));
    }
    
    match dir {
        Direction::N => {
            dirs.push((Direction::W, 2));
            dirs.push((Direction::E, 2));
        }
        Direction::E => {
            dirs.push((Direction::N, 2));
            dirs.push((Direction::S, 2));
        }
        Direction::S => {
            dirs.push((Direction::W, 2));
            dirs.push((Direction::E, 2));
        }
        Direction::W => {
            dirs.push((Direction::N, 2));
            dirs.push((Direction::S, 2));
        }
    }

    dirs
}

fn solve(lines: &[String]) -> usize {
    let mut q: BinaryHeap<Reverse<(usize, Direction, usize, usize, usize)>> = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize, Direction, usize)> = HashSet::new();

    let n = lines.len();
    let m = lines[0].len();

    let loss_at = |i: usize, j: usize| -> usize { (lines[i].as_bytes()[j] - b'0') as usize };

    q.push(Reverse((0, Direction::E, 3, 0, 0)));

    while !q.is_empty() {
        let Reverse((cost, dir, momentum, i, j)) = q.pop().unwrap();

        if visited.contains(&(i, j, dir, momentum)) {
            continue;
        }
        visited.insert((i, j, dir, momentum));

        if i == n - 1 && j == m - 1 {
            return cost;
        }

        for (ndir, nmom) in directions(dir, momentum).into_iter() {
            if ndir == Direction::N && i > 0 {
                q.push(Reverse((cost + loss_at(i - 1, j), ndir, nmom, i - 1, j)));
            } else if ndir == Direction::E && j + 1 < m {
                q.push(Reverse((cost + loss_at(i, j + 1), ndir, nmom, i, j + 1)));
            } else if ndir == Direction::S && i + 1 < n {
                q.push(Reverse((cost + loss_at(i + 1, j), ndir, nmom, i + 1, j)));
            } else if ndir == Direction::W && j > 0 {
                q.push(Reverse((cost + loss_at(i, j - 1), ndir, nmom, i, j - 1)));
            }
        }
    }

    0
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
        let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 102);
    }

    #[test]
    fn test_solution2() {
        let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
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
