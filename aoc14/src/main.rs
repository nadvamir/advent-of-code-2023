use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn tilt_row(row: &mut String, cache: &mut HashMap<String, String>) {
    if let Some(memoised) = cache.get(row) {
        for (i, c) in memoised.bytes().enumerate() {
            unsafe {
                row.as_bytes_mut()[i] = c;
            }
        }
        return;
    }

    let key = row.clone();

    let mut i = 0;
    while i < row.len() {
        let begin_it = i;
        let mut n_stones = 0;

        // find how many movable rows we have
        while i < row.len() {
            match row.as_bytes()[i] {
                b'O' => n_stones += 1,
                b'#' => break,
                _ => {}
            }
            i += 1;
        }

        // fill out stones followed by empty spaces
        for fill_i in begin_it..begin_it + n_stones {
            unsafe {
                row.as_bytes_mut()[fill_i] = b'O';
            }
        }
        for fill_i in begin_it + n_stones..i {
            unsafe {
                row.as_bytes_mut()[fill_i] = b'.';
            }
        }

        i += 1;
    }

    cache.insert(key, row.clone());
}

fn tilt(canvas: &mut Vec<String>, cache: &mut HashMap<String, String>) {
    for i in 0..canvas.len() {
        tilt_row(&mut canvas[i], cache);
    }
}

fn rotate_cw(canvas: &mut Vec<String>) {
    let n = canvas.len();
    for i in 0..n / 2 {
        for j in 0..n / 2 {
            unsafe {
                let c = canvas[i].as_bytes_mut()[j];
                canvas[i].as_bytes_mut()[j] = canvas[n - j - 1].as_bytes_mut()[i];
                canvas[n - j - 1].as_bytes_mut()[i] = canvas[n - i - 1].as_bytes_mut()[n - j - 1];
                canvas[n - i - 1].as_bytes_mut()[n - j - 1] = canvas[j].as_bytes_mut()[n - i - 1];
                canvas[j].as_bytes_mut()[n - i - 1] = c;
            }
        }
    }
}

fn solve(lines: &[String]) -> usize {
    let mut canvas: Vec<String> = lines.iter().map(|s| (*s).clone()).collect();
    let mut cache: HashMap<String, String> = Default::default();

    rotate_cw(&mut canvas);
    rotate_cw(&mut canvas);
    rotate_cw(&mut canvas);
    tilt(&mut canvas, &mut cache);
    rotate_cw(&mut canvas);

    canvas
        .iter()
        .enumerate()
        .map(|(i, line)| line.chars().filter(|c| *c == 'O').count() * (lines.len() - i))
        .sum()
}

// ----------------------------------------------------------------------------
fn tilt_full(canvas: &mut Vec<String>, cache: &mut HashMap<String, String>) {
    rotate_cw(canvas);
    rotate_cw(canvas);
    rotate_cw(canvas);
    tilt(canvas, cache);
    rotate_cw(canvas);
    tilt(canvas, cache);
    rotate_cw(canvas);
    tilt(canvas, cache);
    rotate_cw(canvas);
    tilt(canvas, cache);
    rotate_cw(canvas);
    rotate_cw(canvas);
}

fn solve2(lines: &[String]) -> usize {
    let mut canvas: Vec<String> = lines.iter().map(|s| (*s).clone()).collect();
    let mut cache: HashMap<String, String> = Default::default();
    let mut cycle_cache: HashMap<String, usize> = Default::default();

    let mut i = 0;

    let (cycle_start, cycle_len) = loop {
        let flattened = canvas.concat();
        if let Some(cycle_start) = cycle_cache.get(&flattened) {
            break (cycle_start, i - cycle_start);
        }
        cycle_cache.insert(flattened, i);

        tilt_full(&mut canvas, &mut cache);
        i += 1;
    };

    let nth_iter = cycle_start + (1000000000 - cycle_start) % cycle_len;
    let (final_canvas, _) = cycle_cache.iter().find(|(_, v)| **v == nth_iter).unwrap();

    final_canvas
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if c == 'O' {
                canvas.len() - i / canvas.len()
            } else {
                0
            }
        })
        .sum()
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
        let result = solve2(&lines);
        assert_eq!(result, 64);
    }

    #[test]
    fn test_full_cycle() {
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

        let mut canvas: Vec<String> = lines.iter().map(|s| (*s).clone()).collect();
        let mut cache: HashMap<String, String> = Default::default();

        tilt_full(&mut canvas, &mut cache);

        let output = r".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
";
        let lines: Vec<String> = output.lines().map(|line| line.to_string()).collect();
        let expected_canvas: Vec<String> = lines.iter().map(|s| (*s).clone()).collect();
        assert_eq!(canvas, expected_canvas);
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
