use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;

fn part1() -> i32 {
    let mut res: i32 = 0;
    let re = Regex::new(r"\d+").unwrap();
    if let lines = read_lines("input") {
        for row_idx in 0..lines.len() {
            let line = lines.get(row_idx).unwrap();
            for cap in re.captures_iter(&line) {
                let cap_num = cap.get(0).unwrap();
                let col_idx = cap_num.start() as i32;
                let num_str = cap_num.as_str();
                let mut valid_num = false;

                for col in (col_idx - 1)..(col_idx + num_str.len() as i32 + 1) {
                    if row_idx > 0
                        && lines
                            .get(row_idx - 1)
                            .unwrap()
                            .chars()
                            .nth(col as usize)
                            .unwrap_or('.')
                            != '.'
                    {
                        valid_num = true;
                    }
                    if row_idx < lines.len() - 1
                        && lines
                            .get(row_idx + 1)
                            .unwrap()
                            .chars()
                            .nth(col as usize)
                            .unwrap_or('.')
                            != '.'
                    {
                        valid_num = true
                    }
                }
                if lines
                    .get(row_idx)
                    .unwrap()
                    .chars()
                    .nth((col_idx - 1) as usize)
                    .unwrap_or('.')
                    != '.'
                    || lines
                        .get(row_idx)
                        .unwrap()
                        .chars()
                        .nth((col_idx + num_str.len() as i32) as usize)
                        .unwrap_or('.')
                        != '.'
                {
                    valid_num = true
                }
                if valid_num {
                    res += num_str.parse::<i32>().unwrap();
                }
            }
        }
    }
    res
}

fn part2() -> i32 {
    let mut res: i32 = 0;
    let mut map: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let re = Regex::new(r"\d+").unwrap();
    if let lines = read_lines("input") {
        for row_idx in 0..lines.len() {
            let line = lines.get(row_idx).unwrap();
            for cap in re.captures_iter(&line) {
                let cap_num = cap.get(0).unwrap();
                let col_idx = cap_num.start() as i32;
                let num_str = cap_num.as_str();
                let mut gear_row: i32 = -1;
                let mut gear_col: i32 = -1;

                for col in (col_idx - 1)..(col_idx + num_str.len() as i32 + 1) {
                    if row_idx > 0
                        && lines
                            .get(row_idx - 1)
                            .unwrap()
                            .chars()
                            .nth(col as usize)
                            .unwrap_or('.')
                            == '*'
                    {
                        gear_row = (row_idx - 1) as i32;
                        gear_col = col
                    }
                    if row_idx < lines.len() - 1
                        && lines
                            .get(row_idx + 1)
                            .unwrap()
                            .chars()
                            .nth(col as usize)
                            .unwrap_or('.')
                            == '*'
                    {
                        gear_row = (row_idx + 1) as i32;
                        gear_col = col as i32;
                    }
                }
                if lines
                    .get(row_idx)
                    .unwrap()
                    .chars()
                    .nth((col_idx - 1) as usize)
                    .unwrap_or('.')
                    == '*'
                {
                    gear_row = row_idx as i32;
                    gear_col = col_idx - 1;
                }
                if lines
                    .get(row_idx)
                    .unwrap()
                    .chars()
                    .nth((col_idx + num_str.len() as i32) as usize)
                    .unwrap_or('.')
                    == '*'
                {
                    gear_row = row_idx as i32;
                    gear_col = col_idx + num_str.len() as i32;
                }
                if gear_row >= 0 && gear_col >= 0 {
                    let key = (gear_row, gear_col);
                    let num: i32 = num_str.parse().unwrap();
                    map.entry(key).or_insert(vec![]).push(num);
                }
            }
        }
    }
    for v in map.values() {
        if v.len() == 2 {
            res += v[0] * v[1];
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
