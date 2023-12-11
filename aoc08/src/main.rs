use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn parse_network(lines: &[String]) -> HashMap<String, HashMap<char, String>> {
    let mut network: HashMap<String, HashMap<char, String>> = Default::default();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for line in lines {
        if let Some(caps) = re.captures(line) {
            let from = caps.get(1).unwrap().as_str().to_string();
            let left = caps.get(2).unwrap().as_str().to_string();
            let right = caps.get(3).unwrap().as_str().to_string();
            let directions: HashMap<char, String> =
                vec![('L', left), ('R', right)].into_iter().collect();
            network.insert(from, directions);
        }
    }
    network
}

fn solve(lines: &[String]) -> usize {
    let directions = lines[0].to_string();
    let network = parse_network(&lines[2..]);

    let mut num_iterations: usize = 0;
    let mut current = "AAA".to_string();
    while current != "ZZZ" {
        let step = directions
            .chars()
            .nth(num_iterations % directions.len())
            .unwrap();
        current = network[&current][&step].clone();
        num_iterations += 1;
    }

    num_iterations
}

// ----------------------------------------------------------------------------
fn calc_individual_paths(
    start: &String,
    directions: &String,
    network: &HashMap<String, HashMap<char, String>>,
) -> (usize, Vec<usize>) {
    let mut current = start;
    let mut visited: HashMap<(&String, usize), usize> = Default::default();
    let mut num_iterations: usize = 0;
    let mut possible_endpoints: Vec<usize> = Default::default();

    let next_step = |it| {
        directions
            .chars()
            .nth(it % directions.len())
            .unwrap()
    };

    while !visited.contains_key(&(current, num_iterations % directions.len())) {
        visited.insert((current, num_iterations % directions.len()), num_iterations);
        if current.ends_with('Z') {
            possible_endpoints.push(num_iterations);
        }

        let step = next_step(num_iterations);
        current = &network[current][&step];
        num_iterations += 1;
    }

    let pre_cycle = *visited.get(&(current, num_iterations % directions.len())).unwrap();
    let cycle_len = num_iterations - pre_cycle;
    // assume that we always have a cycle, and thus filter out all possibilities that precede it
    possible_endpoints = possible_endpoints.into_iter().filter(|e| *e >= pre_cycle).collect();
    (cycle_len, possible_endpoints)
}

fn solve2(lines: &[String]) -> usize {
    let directions = lines[0].to_string();
    let network = parse_network(&lines[2..]);

    let current: Vec<&String> = network.keys().filter(|k| k.ends_with('A')).collect();
    let playouts: Vec<(usize, Vec<usize>)> = current
        .iter()
        .map(|c| calc_individual_paths(c, &directions, &network))
        .collect();

    let (max_cycle, candidates) = playouts.iter().max_by(|(a, _), (b, _)| a.cmp(b)).unwrap();

    let terminated = |num_iteration| {
        let any_terminate = |max_cycle, candidates: &Vec<usize>| {
            candidates.iter().any(|c| (num_iteration - c) % max_cycle == 0)
        };
        playouts.iter().all(|(max_cycle, candidates)| any_terminate(max_cycle, candidates))
    };

    let mut iteration: usize = 0;
    let mut candidate: usize = 0;
    while !terminated(candidates.get(candidate).unwrap() + iteration * max_cycle) {
        candidate += 1;
        if candidate == candidates.len() {
            candidate = 0;
            iteration += 1;
        }
    }

    candidates.get(candidate).unwrap() + iteration * max_cycle
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_individual_paths() {
        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let directions = lines[0].to_string();
        let network = parse_network(&lines[2..]);
        let (cycle_len, _) =
            calc_individual_paths(&"11A".to_string(), &directions, &network);
        assert_eq!(cycle_len, 2);
        let (cycle_len, _) =
            calc_individual_paths(&"22A".to_string(), &directions, &network);
        assert_eq!(cycle_len, 6);
    }

    #[test]
    fn test_solution1_1() {
        let input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solution1_2() {
        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_solution2_1() {
        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 6);
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
