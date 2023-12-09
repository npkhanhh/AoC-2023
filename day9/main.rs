use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1() -> i32 {
    let mut res: i32 = 0;
    if let lines = read_lines("input") {
        for line in lines {
            let mut arr: Vec<i32> = line.split(" ").map(|a| a.parse::<i32>().unwrap()).collect();
            let mut stack: Vec<Vec<i32>> = Vec::new();
            stack.push(arr);
            while true {
                let mut new_arr: Vec<i32> = Vec::new();
                for i in 1..stack[stack.len() - 1].len() {
                    new_arr.push(stack[stack.len() - 1][i] - stack[stack.len() - 1][i - 1]);
                }
                stack.push(new_arr.clone());
                let mut can_break = true;
                for e in new_arr {
                    if e != 0 {
                        can_break = false;
                        break;
                    }
                }
                if can_break {
                    break;
                }
            }
            for i in (1..stack.len()).rev() {
                let last_ele = *stack[i].last().unwrap();
                let prev_last = *stack[i - 1].last().unwrap();
                stack[i - 1].push(last_ele + prev_last);
            }
            res += stack[0].last().unwrap();
        }
    }
    res
}

fn part2() -> i32 {
    let mut res: i32 = 0;
    if let lines = read_lines("input") {
        for line in lines {
            let mut arr: Vec<i32> = line.split(" ").map(|a| a.parse::<i32>().unwrap()).collect();
            let mut stack: Vec<Vec<i32>> = Vec::new();
            stack.push(arr);
            while true {
                let mut new_arr: Vec<i32> = Vec::new();
                for i in 1..stack[stack.len() - 1].len() {
                    new_arr.push(stack[stack.len() - 1][i] - stack[stack.len() - 1][i - 1]);
                }
                stack.push(new_arr.clone());
                let mut can_break = true;
                for e in new_arr {
                    if e != 0 {
                        can_break = false;
                        break;
                    }
                }
                if can_break {
                    break;
                }
            }
            for i in (1..stack.len()).rev() {
                let first_ele = *stack[i].first().unwrap();
                let prev_first = *stack[i - 1].first().unwrap();
                stack[i - 1].insert(0, prev_first - first_ele);
            }
            res += stack[0].first().unwrap();
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
