use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Default, PartialEq, Debug)]
struct Marbles {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Default, PartialEq, Debug)]
struct Game {
    id: i32,
    draws: Vec<Marbles>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.draws
            .iter()
            .all(|d| d.red <= 12 && d.green <= 13 && d.blue <= 14)
    }

    fn min_marbles(&self) -> Marbles {
        let mut mmarb: Marbles = Default::default();
        for m in self.draws.iter() {
            mmarb.red = max(mmarb.red, m.red);
            mmarb.green = max(mmarb.green, m.green);
            mmarb.blue = max(mmarb.blue, m.blue);
        }
        mmarb
    }

    fn game_power(&self) -> i32 {
        let mmarb = self.min_marbles();
        mmarb.red * mmarb.green * mmarb.blue
    }
}

fn collect_marble(marbles: &mut Marbles, colour: &str, n: i32) {
    match colour {
        "red" => marbles.red = n,
        "green" => marbles.green = n,
        "blue" => marbles.blue = n,
        &_ => {}
    }
}

fn parse_marbles(marble_draw: &&str) -> Marbles {
    let mut marbles: Marbles = Default::default();
    for draw in marble_draw.split(',') {
        let marble: Vec<&str> = draw.split_whitespace().collect();
        match marble[0].parse::<i32>() {
            Ok(n) => collect_marble(&mut marbles, marble[1], n),
            Err(_) => {}
        }
    }
    return marbles;
}

fn parse_line(line: &str) -> Game {
    if line.is_empty() {
        return Default::default();
    }
    let games: Vec<&str> = line.split(':').collect();
    let name: Vec<&str> = games[0].split_whitespace().collect();

    let id = name[1].parse::<i32>().unwrap_or(0);
    let draws: Vec<&str> = games[1].split(';').collect();

    Game {
        id: id,
        draws: draws.iter().map(parse_marbles).collect(),
    }
}

fn solve(lines: &[String]) -> i32 {
    lines
        .iter()
        .map(|s| parse_line(s))
        .filter(|g| g.is_possible())
        .map(|g| g.id)
        .sum()
}

// ----------------------------------------------------------------------------
fn solve2(lines: &[String]) -> i32 {
    lines
        .iter()
        .map(|s| parse_line(s).game_power())
        .sum()
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let game = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(game.id, 1);
        assert_eq!(
            game.draws,
            [
                Marbles {
                    red: 4,
                    green: 0,
                    blue: 3
                },
                Marbles {
                    red: 1,
                    green: 2,
                    blue: 6
                },
                Marbles {
                    red: 0,
                    green: 2,
                    blue: 0
                }
            ]
        );
    }

    #[test]
    fn test_min_marbles() {
        let game = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(game.id, 1);
        assert_eq!(
            game.min_marbles(),
            Marbles {
                red: 4,
                green: 2,
                blue: 6
            }
        );
    }

    #[test]
    fn test_solution1() {
        let input = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve(&lines);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_solution2() {
        let input = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = solve2(&lines);
        assert_eq!(result, 2286);
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
