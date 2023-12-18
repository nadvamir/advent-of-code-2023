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

fn vertical(mirror: &Vec<&String>, banned: usize) -> usize {
    let mut axes: HashSet<usize> = (0..mirror[0].len() - 1).collect();
    axes.remove(&banned);
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

fn horizontal(mirror: &Vec<&String>, banned: usize) -> usize {
    let mut axes: HashSet<usize> = (0..mirror.len() - 1).collect();
    axes.remove(&banned);
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

    if axes.is_empty() || axes.len() > 1 {
        return 0;
    } else {
        return *axes.iter().next().unwrap() + 1;
    }
}

fn calc_reflection(mirror: &Vec<&String>) -> usize {
    vertical(mirror, 1000) + horizontal(mirror, 1000) * 100
}

fn solve(lines: &[String]) -> usize {
    lines_into_mirrors(lines).iter().map(calc_reflection).sum()
}

// ----------------------------------------------------------------------------
fn calc_reflection_smudged(mirror: &Vec<&String>) -> usize {
    let (ov, oh) = (vertical(mirror, 1000), horizontal(mirror, 1000));
    let (ov, oh) = (if ov > 0 {ov} else {1000}, if oh > 0 { oh } else { 1000 });
    let flip = |c| if c == b'.' { b'#' } else { b'.' };

    let canvas: Vec<String> = mirror.iter().map(|s| (*s).clone()).collect();

    for i in 0..canvas.len() {
        for j in 0..canvas[i].len() {
            let mut line = canvas[i].clone();
            let bytes = unsafe { line.as_bytes_mut() };
            bytes[j] = flip(bytes[j]);

            let mut temp_canvas = canvas.clone();
            temp_canvas[i] = line;

            let new_mirror: Vec<&String> = temp_canvas.iter().collect();
            let (nv, nh) = (vertical(&new_mirror, ov-1), horizontal(&new_mirror, oh-1));

            if nv > 0 || nh > 0 {
                return nv + nh * 100;
            }
        }
    }

    0
}
fn solve2(lines: &[String]) -> usize {
    lines_into_mirrors(lines)
        .iter()
        .map(calc_reflection_smudged)
        .sum()
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
        assert_eq!(result, 400);
    }

    #[test]
    fn test_solution2_2() {
        let input = r"#..##..#..##.
#.####.##.##.
.#.##.#.#....
.#.##.#..####
###..#.#.....
..#..#..##..#
.#.##.#..#..#
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 4);
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
