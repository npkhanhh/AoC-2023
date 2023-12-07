use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line_to_ints(line: String) -> Vec<i32> {
    line.trim()
        .split(" ")
        .filter(|s| !s.is_empty() && *s != " ")
        .map(|e| e.parse::<i32>().unwrap())
        .collect()
}

fn part1() -> i32 {
    let mut res: i32 = 1;
    let mut times: Vec<i32> = Vec::new();
    let mut distances: Vec<i32> = Vec::new();
    if let lines = read_lines("input") {
        times = parse_line_to_ints(lines[0].replace("Time: ", ""));
        distances = parse_line_to_ints(lines[1].replace("Distance: ", ""));
        for round in 0..times.len() {
            let mut number_of_ways = 0;
            for hold in 0..(times[round] + 1) {
                if (times[round] - hold) * hold > distances[round] {
                    number_of_ways += 1;
                }
            }
            res *= number_of_ways;
        }
    }
    res
}

fn part2_bruteforce() -> i64 {
    let mut res: i64 = 0;
    if let lines = read_lines("input") {
        let time = lines[0]
            .replace("Time: ", "")
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty() && *s != " ")
            .collect::<Vec<_>>()
            .join("")
            .parse::<i64>()
            .unwrap();
        let distance = lines[1]
            .replace("Distance: ", "")
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty() && *s != " ")
            .collect::<Vec<_>>()
            .join("")
            .parse::<i64>()
            .unwrap();
        let mut number_of_ways = 0;
        for hold in 0..time + 1 {
            if (time - hold) * hold > distance {
                number_of_ways += 1;
            }
        }
        res = number_of_ways;
    }

    res
}

fn main() {
    // File hosts.txt must exist in the current path
    let p1 = part1();
    println!("part1 {}", p1);
    let p2 = part2_bruteforce();
    println!("part2_bf {}", p2);
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no_file");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("could not parse file"))
        .collect()
}
