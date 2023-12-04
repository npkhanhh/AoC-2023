use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;

fn part1() -> i32 {
    let mut res: i32 = 0;
    if let lines = read_lines("input") {
        for line in lines.to_vec() {
            let (first_with_id, second_part) = line.trim().split_once(" | ").unwrap();
            let (_, first_part) = first_with_id.trim().split_once(": ").unwrap();

            let first_set: HashSet<i32> = first_part
                .trim()
                .replace("  ", " ")
                .split(' ')
                .map(|a| a.trim().parse::<i32>().unwrap())
                .collect();
            let second_set: HashSet<i32> = second_part
                .trim()
                .replace("  ", " ")
                .split(' ')
                .map(|a| a.trim().parse::<i32>().unwrap())
                .collect();
            let inter = first_set.intersection(&second_set);
            let count = inter.count() as u32;
            if count > 0 {
                res += i32::pow(2, count - 1);
            }
        }
    }
    res
}

fn part2() -> i32 {
    let mut res: i32 = 0;
    if let lines = read_lines("input") {
        let n = lines.len();
        let mut counts: Vec<i32> = vec![1; n];
        for (idx, line) in lines.into_iter().enumerate() {
            let (first_with_id, second_part) = line.trim().split_once(" | ").unwrap();
            let (_, first_part) = first_with_id.trim().split_once(": ").unwrap();

            let first_set: HashSet<i32> = first_part
                .trim()
                .replace("  ", " ")
                .split(' ')
                .map(|a| a.trim().parse::<i32>().unwrap())
                .collect();
            let second_set: HashSet<i32> = second_part
                .trim()
                .replace("  ", " ")
                .split(' ')
                .map(|a| a.trim().parse::<i32>().unwrap())
                .collect();
            let inter = first_set.intersection(&second_set);
            let inter_count = inter.count();
            let count = counts[idx];
            res += count;
            if inter_count > 0 {
                for i in idx + 1..std::cmp::min(idx + 1 + inter_count, n) {
                    counts[i] += count;
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
