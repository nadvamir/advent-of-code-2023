use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_stars(lines: &[String]) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.as_bytes().iter().enumerate() {
            if *c == b'#' {
                res.push((i, j));
            }
        }
    }
    res
}

fn expand<F, FM>(stars: &mut Vec<(usize, usize)>, coord_accessor: F, coord_accessor_mut: &mut FM) 
where
    F: Fn(&(usize, usize)) -> &usize,
    FM: FnMut(&mut (usize, usize)) -> &mut usize
{
    stars.sort_by(|a, b| coord_accessor(a).cmp(coord_accessor(b)));
    let mut shift: usize = 0;
    let mut last: usize = 0;
    for star in stars.iter_mut() {
        let coord = coord_accessor_mut(star);
        if let Some(diff) = coord.checked_sub(last + 1) {
            shift += diff;
        }
        last = *coord;
        *coord += shift;
    }
}

fn manhattan_dist(s1: &(usize, usize), s2: &(usize, usize)) -> usize {
    let ((i1, j1), (i2, j2)) = ((s1.0 as i32, s1.1 as i32), (s2.0 as i32, s2.1 as i32));
    ((i1 - i2).abs() + (j1 - j2).abs()) as usize
}

fn sum_dists(stars: &Vec<(usize, usize)>) -> usize {
    let mut dist: usize = 0;
    for a in 0..stars.len() {
        for b in a+1..stars.len() {
            dist += manhattan_dist(&stars[a], &stars[b]);
        }
    }
    dist
}

fn solve(lines: &[String]) -> usize {
    let mut stars = parse_stars(lines);
    expand(&mut stars, |(i, _)| i, &mut |(i, _)| i);
    expand(&mut stars, |(_, j)| j, &mut |(_, j)| j);
    sum_dists(&stars)
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
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 374);
    }
    #[test]
    fn test_solution2() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
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
