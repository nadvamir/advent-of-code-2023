use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Hand {
    power: usize,
    cards: String,
    bid: usize,
}

lazy_static! {
    static ref MAPPING: Vec<(char, i64)> = vec![
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ];
    static ref CARD_VALS: HashMap<char, i64> = MAPPING.iter().map(|&(c, v)| (c, v)).collect();
}

fn compare(a: &Hand, b: &Hand) -> Ordering {
    if a.power == b.power {
        for (l, r) in a.cards.chars().zip(b.cards.chars()) {
            if CARD_VALS.get(&l).unwrap() < CARD_VALS.get(&r).unwrap() {
                return Ordering::Less;
            } else if CARD_VALS.get(&l).unwrap() > CARD_VALS.get(&r).unwrap() {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    } else {
        a.power.cmp(&b.power)
    }
}

fn calc_power(cards: &str) -> usize {
    let mut map: HashMap<char, usize> = Default::default();
    for c in cards.chars() {
        map.insert(c, map.get(&c).unwrap_or(&0) + 1);
    }
    match map.len() {
        1 => 6,
        2 => {
            if *map.values().max().unwrap() == 4 {
                5
            } else {
                4
            }
        }
        3 => {
            if *map.values().max().unwrap() == 3 {
                3
            } else {
                2
            }
        }
        4 => 1,
        _ => 0,
    }
}

fn parse_hand(line: &str) -> Hand {
    let line: Vec<&str> = line.split_whitespace().collect();
    let (hand, bid) = (line[0], line[1]);
    Hand {
        power: calc_power(hand),
        cards: hand.to_string(),
        bid: bid.parse::<usize>().unwrap(),
    }
}

fn solve(lines: &[String]) -> usize {
    let mut hands: Vec<Hand> = lines.iter().map(|s| parse_hand(s)).collect();
    hands.sort_by(|a, b| compare(a, b));
    let (_, res) = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i, h.bid))
        .reduce(|(_, bids), (i, bid)| (i, bids + (i + 1) * bid))
        .unwrap();
    res
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
    fn test_calc_power() {
        assert_eq!(calc_power("AAAAA"), 6);
        assert_eq!(calc_power("AAJAA"), 5);
        assert_eq!(calc_power("AJJAA"), 4);
        assert_eq!(calc_power("AJ9AA"), 3);
        assert_eq!(calc_power("AJ99A"), 2);
        assert_eq!(calc_power("AJ98A"), 1);
        assert_eq!(calc_power("AJ98T"), 0);
    }

    #[test]
    fn test_solution1() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_solution2() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
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
