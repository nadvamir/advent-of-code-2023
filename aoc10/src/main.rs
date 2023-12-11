use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Default, Clone)]
struct Pipe {
    connections: Vec<(usize, usize)>,
    dist: usize,
}

fn parse_connections(lines: &[String]) -> Vec<Vec<Pipe>> {
    let (n, m) = (lines.len(), lines[0].len());
    let mut pipes: Vec<Vec<Pipe>> = vec![vec![Default::default(); m]; n];

    let mut record = |i: usize, j: usize, di: isize, dj: isize| {
        if let (Some(ni), Some(nj)) = (i.checked_add_signed(di), j.checked_add_signed(dj)) {
            if ni >= n || nj >= m || (ni == i && nj == j) {
                return;
            }
            pipes[i][j].connections.push((ni, nj));
            if *lines[ni].as_bytes().get(nj).unwrap() == b'S' {
                pipes[ni][nj].connections.push((i, j));
            }
        }
    };

    for i in 0..n {
        for j in 0..m {
            let (di1, dj1, di2, dj2) = match lines[i].as_bytes().get(j) {
                Some(b'|') => (-1, 0, 1, 0),
                Some(b'-') => (0, -1, 0, 1),
                Some(b'L') => (-1, 0, 0, 1),
                Some(b'J') => (-1, 0, 0, -1),
                Some(b'7') => (1, 0, 0, -1),
                Some(b'F') => (1, 0, 0, 1),
                _ => (0, 0, 0, 0)
            };
            record(i, j, di1, dj1);
            record(i, j, di2, dj2);
        }
    }
    pipes
}

fn find_start(lines: &[String]) -> (usize, usize) {
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn max_fill(pipes: &mut Vec<Vec<Pipe>>, si: usize, sj: usize) -> usize {
    let mut q = VecDeque::new();
    q.push_back((si, sj, 0));

    let mut max_d = 0;

    while !q.is_empty() {
        let (i, j, d) = q.pop_front().unwrap();
        if pipes[i][j].dist > 0 {
            continue;
        }
        pipes[i][j].dist = d;
        max_d = std::cmp::max(max_d, d);
        for (ni, nj) in pipes[i][j].connections.iter() {
            q.push_back((*ni, *nj, d + 1));
        }
    }
    max_d
}

fn solve(lines: &[String]) -> usize {
    let mut pipes = parse_connections(lines);
    let (si, sj) = find_start(lines);
    max_fill(&mut pipes, si, sj)
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
    fn test_solution1_1() {
        let input = r".....
.S-7.
.|.|.
.L-J.
.....
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_solution1_2() {
        let input = r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_solution1_3() {
        let input = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 8);
    }


    #[test]
    fn test_solution2_1() {
        let input = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_solution2_2() {
        let input = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_solution2_3() {
        let input = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 8);
    }
    #[test]
    fn test_solution2_4() {
        let input = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 10);
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
