use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn ascii_hash(text: &str) -> usize {
    text.as_bytes()
        .iter()
        .fold(0, |acc, &c| (acc + c as usize) * 17 % 256) as usize
}

fn solve(lines: &[String]) -> usize {
    lines[0].split(',').map(|s| ascii_hash(s)).sum()
}

// ----------------------------------------------------------------------------
fn solve2(lines: &[String]) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];

    for cmd in lines[0].split(',') {
        if let Some(idx) = cmd.find('=') {
            let tag = &cmd[..idx];
            let box_idx = ascii_hash(tag);
            let fl = cmd[idx+1..].parse::<usize>().expect("Focal length must be a number");
            if let Some((_, stored_fl)) = boxes[box_idx].iter_mut().find(|(stored_tag, _)| stored_tag == tag) {
                *stored_fl = fl;
            }
            else {
                boxes[box_idx].push((tag.to_string(), fl));
            }
        }
        else {
            let tag = &cmd[..cmd.len()-1];
            let box_idx = ascii_hash(tag);
            boxes[box_idx].retain(|(stored_tag, _)| stored_tag != tag);
        }
    }

    let calc_box = |idx, b: &Vec<(String, usize)>| -> usize {
        b.iter().enumerate().map(|(b_idx, (_, fl))| (idx + 1) * (b_idx + 1) * fl).sum()
    };

    boxes.iter().enumerate().map(|(idx, b)| calc_box(idx, b)).sum()
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_solution2() {
        let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 145);
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
