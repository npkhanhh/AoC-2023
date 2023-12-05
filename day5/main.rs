use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line_to_ints(line: String) -> Vec<i64> {
    line.split(" ").map(|e| e.parse::<i64>().unwrap()).collect()
}

fn part1() -> i64 {
    let mut res: i64 = i64::MAX;
    let mut map: Vec<Vec<Vec<i64>>> = Vec::new();
    let mut map_index: i32 = -1;
    let mut seeds: Vec<i64> = Vec::new();
    if let lines = read_lines("input") {
        for (idx, line) in lines.into_iter().enumerate() {
            if line.len() == 0 {
                continue;
            }
            if idx == 0 {
                seeds = parse_line_to_ints(line.replace("seeds: ", ""));
                continue;
            }
            if line.contains("map") {
                map.push(Vec::new());
                map_index += 1;
                continue;
            }
            let num_line = parse_line_to_ints(line);
            match map[map_index as usize].binary_search_by_key(&num_line[1], |v| v[1]) {
                Ok(_) => {}
                Err(pos) => map[map_index as usize].insert(pos, num_line),
            }
        }
    }
    for seed in seeds {
        let mut t = seed;
        for m in &map {
            match m.binary_search_by_key(&t, |v| v[1]) {
                Ok(pos) => t = m[pos][0] + (t - m[pos][1]),
                Err(pos) => t = m[pos - 1][0] + (t - m[pos - 1][1]),
            }
        }
        res = std::cmp::min(res, t);
    }
    res
}

fn part2() -> i64 {
    let mut res: i64 = i64::MAX;
    let mut map: Vec<Vec<Vec<i64>>> = Vec::new();
    let mut map_index: i32 = -1;
    let mut seeds: Vec<i64> = Vec::new();
    if let lines = read_lines("input") {
        for (idx, line) in lines.into_iter().enumerate() {
            if line.len() == 0 {
                continue;
            }
            if idx == 0 {
                seeds = parse_line_to_ints(line.replace("seeds: ", ""));
                continue;
            }
            if line.contains("map") {
                map.push(Vec::new());
                map_index += 1;
                continue;
            }
            let num_line = parse_line_to_ints(line);

            match map[map_index as usize].binary_search_by_key(&num_line[0], |v| v[0]) {
                Ok(_) => {}
                Err(pos) => map[map_index as usize].insert(pos, num_line),
            }
        }
    }
    let mut sorted_seeds: Vec<(i64, i64)> = Vec::new();

    for seed_idx in (0..seeds.len()).step_by(2) {
        match sorted_seeds.binary_search_by_key(&seeds[seed_idx], |&(a, b)| a) {
            Ok(_) => {}
            Err(pos) => sorted_seeds.insert(
                pos,
                (seeds[seed_idx], seeds[seed_idx] + seeds[seed_idx + 1]),
            ),
        }
    }
    let mut seed = 0;
    let mut has_res = false;
    while true {
        let mut t = seed;
        for m in map.clone().into_iter().rev() {
            match m.binary_search_by_key(&t, |v| v[0]) {
                Ok(pos) => t = m[pos][1] + (t - m[pos][0]),
                Err(pos) => t = m[pos - 1][1] + (t - m[pos - 1][0]),
            }
        }
        for &(a, b) in &sorted_seeds {
            if a <= t && t <= b {
                has_res = true;
                break;
            }
        }
        if has_res {
            break;
        }
        seed += 1000;
        if seed % 1000000 == 0 {
            println!("{}", seed);
        }
    }
    for seed in (seed - 1001)..(seed + 1) {
        let mut t = seed;
        for m in map.clone().into_iter().rev() {
            match m.binary_search_by_key(&t, |v| v[0]) {
                Ok(pos) => t = m[pos][1] + (t - m[pos][0]),
                Err(pos) => t = m[pos - 1][1] + (t - m[pos - 1][0]),
            }
        }
        for &(a, b) in &sorted_seeds {
            if a <= t && t <= b {
                return seed;
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
