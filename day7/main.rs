use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn replace_ascii(hand: Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let ascii_map: HashMap<u8, u8> = HashMap::from([
        ('A' as u8, 'Z' as u8),
        ('T' as u8, 'B' as u8),
        ('K' as u8, 'Y' as u8),
        ('J' as u8, '1' as u8),
    ]);
    for c in hand {
        if ascii_map.contains_key(&c) {
            res.push(*ascii_map.get(&c).unwrap());
        } else {
            res.push(c);
        }
    }
    res
}

fn get_hand_rank(hand: Vec<u8>) -> i32 {
    let mut frequencies =
        hand.iter()
            .copied()
            .fold(HashMap::new(), |mut map, val| -> HashMap<u8, i32> {
                *map.entry(val).or_default() += 1;
                map
            });
    let mut j_count = 0;
    if frequencies.contains_key(&('1' as u8)) {
        j_count += frequencies.get(&('1' as u8)).unwrap();
        frequencies.remove(&('1' as u8));
    }
    let mut max_count = 0;
    if frequencies.len() > 0 {
        max_count = *frequencies.values().into_iter().max().unwrap() + j_count;
    }
    if frequencies.len() == 1 || frequencies.len() == 0 {
        return 6;
    } else if frequencies.len() == 2 {
        if max_count == 4 {
            // Four of a kind
            return 5;
        } else if max_count == 3 {
            // Full house
            return 4;
        }
    } else if frequencies.len() == 3 {
        if max_count == 3 {
            // Three of a kind
            return 3;
        } else if max_count == 2 {
            return 2;
        }
    } else if frequencies.len() == 4 {
        // 1 pair
        return 1;
    }
    0
}

fn part2() -> i64 {
    let mut res: i64 = 0;
    let mut hands: Vec<(i32, (u8, u8, u8, u8, u8), i32, String)> = Vec::new();
    if let lines = read_lines("input") {
        for line in lines {
            let mut l = line.clone();
            let (h, rank_str) = l.split_once(" ").unwrap();
            let mut hand = replace_ascii(h.clone().as_bytes().to_vec());
            let hand_rank = get_hand_rank(hand.clone());
            hands.push((
                hand_rank,
                (hand[0], hand[1], hand[2], hand[3], hand[4]),
                rank_str.parse::<i32>().unwrap(),
                h.clone().to_owned(),
            ));
        }
    }
    hands.sort_by_key(|&(a, b, _, _)| (a, b));
    for (idx, (_, _, point, hand)) in hands.iter().enumerate() {
        res += ((idx as i64 + 1) * *point as i64) as i64;
    }
    res
}

fn main() {
    // File hosts.txt must exist in the current path
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
