use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line(line: &String) -> (String, Vec<usize>) {
    let line: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    let consec_springs: Vec<usize> = line[1]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    (line[0].clone(), consec_springs)
}

fn num_arrangements(
    pattern: &str,
    consec_springs: &Vec<usize>,
    (offset_p, offset_c): (usize, usize),
) -> usize {
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    return num_arrangements_memoised(pattern, consec_springs, (offset_p, offset_c), &mut cache);
}

fn num_arrangements_memoised(
    pattern: &str,
    consec_springs: &Vec<usize>,
    (offset_p, offset_c): (usize, usize),
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(memoised) = cache.get(&(offset_p, offset_c)) {
        return *memoised;
    }

    if offset_c == consec_springs.len() {
        return if offset_p >= pattern.len()
            || pattern[offset_p..].chars().all(|c| c == '.' || c == '?')
        {
            1
        } else {
            0
        };
    }
    if offset_p >= pattern.len() {
        return 0;
    }

    let result = (|| {
        let curr_c = pattern.chars().nth(offset_p).unwrap();
        if curr_c == '.' {
            return num_arrangements_memoised(
                pattern,
                consec_springs,
                (offset_p + 1, offset_c),
                cache,
            );
        }

        let consec_width = consec_springs[offset_c];
        let pattern_end = offset_p + consec_width;
        if pattern_end > pattern.len() {
            return 0;
        }

        let mut result = 0;

        if curr_c == '?' {
            result +=
                num_arrangements_memoised(pattern, consec_springs, (offset_p + 1, offset_c), cache);
        }

        if pattern[offset_p..pattern_end]
            .chars()
            .all(|c| c == '#' || c == '?')
        {
            if pattern_end < pattern.len() && *pattern.as_bytes().get(pattern_end).unwrap() != b'#'
                || pattern_end == pattern.len()
            {
                result += num_arrangements_memoised(
                    pattern,
                    consec_springs,
                    (pattern_end + 1, offset_c + 1),
                    cache,
                )
            }
        }

        result
    })();

    cache.insert((offset_p, offset_c), result);
    result
}

fn solve(lines: &[String]) -> usize {
    lines
        .iter()
        .map(parse_line)
        .map(|(p, c)| num_arrangements(&p, &c, (0, 0)))
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
    fn test_num_arrangements() {
        assert_eq!(num_arrangements("??.??", &Vec::from(vec![1, 1]), (0, 0)), 4);
    }

    #[test]
    fn test_solution1_1() {
        let input = r"#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_solution1_2() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_solution2() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
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
