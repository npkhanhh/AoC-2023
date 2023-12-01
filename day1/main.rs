use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let map: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();
    let mut res = 0;
    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ss) = line {
                let s = ss
                    .replace("twone", "twoone")
                    .replace("eightwo", "eighttwo")
                    .replace("eighthree", "eightthree")
                    .replace("threeight", "threeeight")
                    .replace("oneight", "oneeight")
                    .replace("nineight", "nineeight")
                    .replace("fiveight", "fiveeight");
                let captures: Vec<_> = re.captures_iter(&s).collect();
                let first_capture = captures.first().unwrap().get(0).unwrap().as_str();
                let last_capture = captures.last().unwrap().get(0).unwrap().as_str();
                let first: i32;
                let second: i32;
                if map.contains_key(first_capture) {
                    first = *map.get(first_capture).unwrap();
                } else {
                    first = first_capture.parse::<i32>().unwrap();
                };
                if map.contains_key(last_capture) {
                    second = *map.get(last_capture).unwrap();
                } else {
                    second = last_capture.parse::<i32>().unwrap();
                };
                println!("{}", first * 10 + second);
                res += first * 10 + second;
            }
        }
    }
    println!("res: {}", res);
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
