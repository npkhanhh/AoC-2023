use std::collections::HashMap;
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;

fn part1() -> i32 {
    let map: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut res: i32 = 0;
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(s) = line {
                if let Some((game_id, game_content)) = s.split_once(": ") {
                    let id = game_id.split(" ").last().unwrap().parse::<i32>().unwrap();
                    let game = game_content.replace(";", ",");
                    let mut valid_game: bool = true;
                    for ball in game.split(", ") {
                        if let Some((no, color)) = ball.split_once(" ") {
                            let count = no.parse::<i32>().unwrap();
                            if count > *map.get(color).unwrap() {
                                valid_game = false;
                            }
                        }
                    }
                    if valid_game {
                        res += id;
                    }
                }
            }
        }
    }
    res
}

fn part2() -> i32 {
    let mut res: i32 = 0;
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(s) = line {
                if let Some((_game_id, game_content)) = s.split_once(": ") {
                    let mut map: HashMap<&str, i32> =
                        HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
                    let game = game_content.replace(";", ",");
                    for ball in game.split(", ") {
                        if let Some((no, color)) = ball.split_once(" ") {
                            let count = no.parse::<i32>().unwrap();
                            if count > *map.get(color).unwrap() {
                                map.insert(color, count);
                            }
                        }
                    }
                    let product = map.values().copied().reduce(|a, b| a * b).unwrap();
                    res += product;
                }
            }
        }
    }
    res
}

fn main() {
    // File hosts.txt must exist in the current path
    let p1 = part1();
    println!("part1 {}", p1);
    let p2 = part2();
    println!("part2 {}", p2);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
