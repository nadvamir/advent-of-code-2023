use std::cmp::min;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::path::Path;

#[derive(Debug, PartialEq, Default)]
struct Seed {
    seed: i64,
    soil: i64,
    fert: i64,
    water: i64,
    light: i64,
    temp: i64,
    humid: i64,
    loc: i64,
}

#[derive(Default)]
struct Mappings {
    seed2soil: BTreeMap<i64, i64>,
    soil2fert: BTreeMap<i64, i64>,
    fert2water: BTreeMap<i64, i64>,
    water2light: BTreeMap<i64, i64>,
    light2temp: BTreeMap<i64, i64>,
    temp2humid: BTreeMap<i64, i64>,
    humid2loc: BTreeMap<i64, i64>,
}

fn translate(mapping: &BTreeMap<i64, i64>, el: &i64) -> i64 {
    let upper_bound = mapping.range((Unbounded, Included(el)));

    match upper_bound.rev().next() {
        Some((from, to)) => to + el - from,
        None => *el,
    }
}

fn parse_seeds(line: &str) -> Vec<i64> {
    line.split_whitespace().collect::<Vec<&str>>()[1..]
        .iter()
        .map(|seed| seed.parse::<i64>().expect("Number"))
        .collect::<Vec<i64>>()
}

fn parse_mapping(lines: &[String], offset: &usize) -> (usize, BTreeMap<i64, i64>) {
    let mut mapping: BTreeMap<i64, i64> = [(0, 0)].into_iter().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.len() == 0 {
            return (offset + 2 + i, mapping);
        }

        let vals: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().expect("Number"))
            .collect();
        let end = vals[1] + vals[2];

        mapping.insert(vals[1], vals[0]);
        if !mapping.contains_key(&end) {
            mapping.insert(end, end);
        }
    }
    (offset + 2 + lines.len(), mapping)
}

fn parse_mappings(lines: &[String]) -> Mappings {
    let mut mappings: Mappings = Default::default();
    let mut offset: usize = 0;
    (offset, mappings.seed2soil) = parse_mapping(&lines[offset + 1..], &offset);
    (offset, mappings.soil2fert) = parse_mapping(&lines[offset + 1..], &offset);
    (offset, mappings.fert2water) = parse_mapping(&lines[offset + 1..], &offset);
    (offset, mappings.water2light) = parse_mapping(&lines[offset + 1..], &offset);
    (offset, mappings.light2temp) = parse_mapping(&lines[offset + 1..], &offset);
    (offset, mappings.temp2humid) = parse_mapping(&lines[offset + 1..], &offset);
    (_, mappings.humid2loc) = parse_mapping(&lines[offset + 1..], &offset);
    mappings
}

fn resolve_seed(seed: &i64, mappings: &Mappings) -> Seed {
    let mut s: Seed = Default::default();
    s.seed = *seed;
    s.soil = translate(&mappings.seed2soil, &s.seed);
    s.fert = translate(&mappings.soil2fert, &s.soil);
    s.water = translate(&mappings.fert2water, &s.fert);
    s.light = translate(&mappings.water2light, &s.water);
    s.temp = translate(&mappings.light2temp, &s.light);
    s.humid = translate(&mappings.temp2humid, &s.temp);
    s.loc = translate(&mappings.humid2loc, &s.humid);
    s
}

fn solve(lines: &[String]) -> i64 {
    let seeds = parse_seeds(&lines[0]);
    let mappings = parse_mappings(&lines[2..]);
    seeds
        .iter()
        .map(|s| resolve_seed(s, &mappings).loc)
        .min()
        .unwrap_or(0)
}

// ----------------------------------------------------------------------------
fn solve2(lines: &[String]) -> i64 {
    let seeds = parse_seeds(&lines[0]);
    let mappings = parse_mappings(&lines[2..]);
    let mut min_loc: i64 = i64::MAX;

    for i in (0..seeds.len()).step_by(2) {
        min_loc = min(
            min_loc,
            mappings
                .seed2soil
                .range((Included(seeds[i]), Excluded(seeds[i] + seeds[i + 1])))
                .map(|(seed, _)| resolve_seed(seed, &mappings).loc)
                .min()
                .unwrap_or(min_loc),
        );
        min_loc = min(min_loc, resolve_seed(&seeds[i], &mappings).loc);
        min_loc = min(min_loc, resolve_seed(&(seeds[i] + seeds[i + 1]), &mappings).loc);
    }

    min_loc
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation() {
        let input = r"50 98 2
52 50 48

";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let (_, mapping) = parse_mapping(&lines, &0);
        assert_eq!(translate(&mapping, &0), 0);
        assert_eq!(translate(&mapping, &1), 1);
        assert_eq!(translate(&mapping, &49), 49);
        assert_eq!(translate(&mapping, &50), 52);
        assert_eq!(translate(&mapping, &51), 53);
        assert_eq!(translate(&mapping, &97), 99);
        assert_eq!(translate(&mapping, &98), 50);
        assert_eq!(translate(&mapping, &99), 51);
    }

    #[test]
    fn test_solution1() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_solution2() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 46);
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
