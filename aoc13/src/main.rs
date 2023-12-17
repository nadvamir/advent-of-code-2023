use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_into_mirrors(lines: &[String]) -> Vec<Vec<&String>> {
    let grouped: Vec<Vec<&String>> = lines.iter().fold(vec![vec![]], |mut acc, line| {
        if line.is_empty() {
            acc.push(vec![]);
        } else {
            acc.last_mut().unwrap().push(&line);
        }
        acc
    });
    grouped.into_iter().filter(|g| !g.is_empty()).collect()
}

fn mirrored(line: &String, pivot: usize) -> bool {
    let mut j = pivot;
    let mut k = pivot + 1;
    let bline = line.as_bytes();
    loop {
        if bline[j] != bline[k] {
            break false;
        }
        if j == 0 || k == bline.len() - 1 {
            break true;
        }
        j -= 1;
        k += 1;
    }
}

fn mirrored_i(mirror: &Vec<&String>, j: usize, pivot: usize) -> bool {
    let mut i = pivot;
    let mut k = pivot + 1;
    loop {
        if mirror[i].as_bytes()[j] != mirror[k].as_bytes()[j] {
            break false;
        }
        if i == 0 || k == mirror.len() - 1 {
            break true;
        }
        i -= 1;
        k += 1;
    }
}

fn vertical(mirror: &Vec<&String>) -> usize {
    let mut axes: HashSet<usize> = (0..mirror[0].len() - 1).collect();
    let mut i: usize = 0;
    while i < mirror.len() && !axes.is_empty() {
        let axes_at_i = axes.clone();
        for j in axes_at_i {
            if !mirrored(mirror[i], j) {
                axes.remove(&j);
            }
        }
        i += 1;
    }

    if axes.is_empty() {
        return 0;
    } else {
        return *axes.iter().next().unwrap() + 1;
    }
}

fn horizontal(mirror: &Vec<&String>) -> usize {
    let mut axes: HashSet<usize> = (0..mirror.len() - 1).collect();
    let mut j: usize = 0;
    while j < mirror[0].len() && !axes.is_empty() {
        let axes_at_j = axes.clone();
        for i in axes_at_j {
            if !mirrored_i(mirror, j, i) {
                axes.remove(&i);
            }
        }
        j += 1;
    }

    if axes.is_empty() {
        return 0;
    } else {
        return *axes.iter().next().unwrap() + 1;
    }
}

fn calc_reflection(mirror: &Vec<&String>) -> usize {
    vertical(mirror) + horizontal(mirror) * 100
}

fn solve(lines: &[String]) -> usize {
    lines_into_mirrors(lines).iter().map(calc_reflection).sum()
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
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 405);
    }

    #[test]
    fn test_solution2() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
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
