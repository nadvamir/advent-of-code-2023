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

fn directions(
    dir: Direction,
    min_mom: usize,
    max_mom: usize,
    min_momentum: usize,
    max_momentum: usize,
) -> Vec<(Direction, usize, usize)> {
    let mut dirs = vec![];

    if min_mom > 0 {
        dirs.push((dir, min_mom - 1, max_mom - 1));
        return dirs;
    }

    if max_mom > 0 {
        dirs.push((dir, 0, max_mom - 1));
    }

    let nmin = if min_momentum > 0 { min_momentum - 1 } else { 0 };
    let nmax = max_momentum - 1;

    match dir {
        Direction::N => {
            dirs.push((Direction::W, nmin, nmax));
            dirs.push((Direction::E, nmin, nmax));
        }
        Direction::E => {
            dirs.push((Direction::N, nmin, nmax));
            dirs.push((Direction::S, nmin, nmax));
        }
        Direction::S => {
            dirs.push((Direction::W, nmin, nmax));
            dirs.push((Direction::E, nmin, nmax));
        }
        Direction::W => {
            dirs.push((Direction::N, nmin, nmax));
            dirs.push((Direction::S, nmin, nmax));
        }
    }

    dirs
}

fn dijkstra(lines: &[String], min_momentum: usize, max_momentum: usize) -> usize {
    let mut q: BinaryHeap<Reverse<(usize, Direction, usize, usize, usize, usize)>> =
        BinaryHeap::new();
    let mut visited: HashSet<(usize, usize, Direction, usize, usize)> = HashSet::new();

    let n = lines.len();
    let m = lines[0].len();

    let loss_at = |i: usize, j: usize| -> usize { (lines[i].as_bytes()[j] - b'0') as usize };

    q.push(Reverse((0, Direction::E, min_momentum, max_momentum, 0, 0)));
    q.push(Reverse((0, Direction::S, min_momentum, max_momentum, 0, 0)));

    while !q.is_empty() {
        let Reverse((cost, dir, min_mom, max_mom, i, j)) = q.pop().unwrap();

        if visited.contains(&(i, j, dir, min_mom, max_mom)) {
            continue;
        }
        visited.insert((i, j, dir, min_mom, max_mom));

        if i == n - 1 && j == m - 1 && min_mom == 0 {
            return cost;
        }

        for (ndir, nmimom, nmamom) in directions(dir, min_mom, max_mom, min_momentum, max_momentum).into_iter() {
            if ndir == Direction::N && i > 0 {
                q.push(Reverse((cost + loss_at(i - 1, j), ndir, nmimom, nmamom, i - 1, j)));
            } else if ndir == Direction::E && j + 1 < m {
                q.push(Reverse((cost + loss_at(i, j + 1), ndir, nmimom, nmamom, i, j + 1)));
            } else if ndir == Direction::S && i + 1 < n {
                q.push(Reverse((cost + loss_at(i + 1, j), ndir, nmimom, nmamom, i + 1, j)));
            } else if ndir == Direction::W && j > 0 {
                q.push(Reverse((cost + loss_at(i, j - 1), ndir, nmimom, nmamom, i, j - 1)));
            }
        }
    }

    0
}

fn solve(lines: &[String]) -> usize {
    dijkstra(lines, 0, 3)
}

// ----------------------------------------------------------------------------
fn solve2(lines: &[String]) -> usize {
    dijkstra(lines, 4, 10)
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
        assert_eq!(result, 94);
    }

    #[test]
    fn test_solution2_2() {
        let input = r"111111111111
999999999991
999999999991
999999999991
999999999991";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 71);
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
