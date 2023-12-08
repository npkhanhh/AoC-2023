use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1() -> usize {
    let mut g: HashMap<String, (String, String)> = HashMap::new();
    let mut lr_seq: String = String::new();
    if let lines = read_lines("input") {
        for (idx, line) in lines.iter().enumerate() {
            if idx == 0 {
                lr_seq = line.to_owned();
                continue;
            }
            if line.len() == 0 {
                continue;
            }
            let (source, dest) = line.split_once(" = ").unwrap().to_owned();
            let dest_str = dest.replace("(", "").replace(")", "").to_owned();

            let (l, r) = dest_str.split_once(", ").unwrap().to_owned();
            g.insert(source.to_owned(), (l.to_owned(), r.to_owned()));
        }
    }
    let mut idx = 0;
    let mut cur = "AAA";
    while cur != "ZZZ" {
        let (l, r) = g.get(cur).unwrap();
        if lr_seq.chars().nth(idx % lr_seq.len()).unwrap() == 'L' {
            cur = l;
        } else {
            cur = r;
        }
        idx += 1;
    }
    idx
}

fn gcd(a: i64, b: i64) -> i64 {
    if a < b {
        return gcd(b, a);
    }
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn part2() -> i64 {
    let mut g: HashMap<String, (String, String)> = HashMap::new();
    let mut lr_seq: String = String::new();
    if let lines = read_lines("input") {
        for (idx, line) in lines.iter().enumerate() {
            if idx == 0 {
                lr_seq = line.to_owned();
                continue;
            }
            if line.len() == 0 {
                continue;
            }
            let (source, dest) = line.split_once(" = ").unwrap().to_owned();
            let dest_str = dest.replace("(", "").replace(")", "").to_owned();

            let (l, r) = dest_str.split_once(", ").unwrap().to_owned();
            g.insert(source.to_owned(), (l.to_owned(), r.to_owned()));
        }
    }
    let starts = g.keys().filter(|k| k.chars().nth(2).unwrap() == 'A');
    let mut steps: Vec<i32> = Vec::new();
    for start in starts {
        let mut idx = 0;
        let mut cur = start;
        while cur.chars().nth(2).unwrap() != 'Z' {
            let (l, r) = g.get(cur).unwrap();
            if lr_seq.chars().nth(idx % lr_seq.len()).unwrap() == 'L' {
                cur = l;
            } else {
                cur = r;
            }
            idx += 1;
        }
        steps.push(idx as i32);
    }
    let mut res: i64 = 1;
    for step in steps {
        res = lcm(res, step as i64);
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
