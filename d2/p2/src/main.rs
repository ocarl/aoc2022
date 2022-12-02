use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashMap;


fn calc_score(them: &str, me: &str) -> u32 {
    if them == "A" && me == "X" {
        return 3u32
    }
    else if them == "A" && me == "Y" {
        return 4u32
    }
    else if them == "A" && me == "Z" {
        return 8u32
    }
    else if them == "B" && me == "X" {
        return 1u32
    }
    else if them == "B" && me == "Y" {
        return 5u32
    }
    else if them == "B" && me == "Z" {
        return 9u32
    }
    else if them == "C" && me == "X" {
        return 2u32
    }
    else if them == "C" && me == "Y" {
        return 6u32
    }
    else if them == "C" && me == "Z" {
        return 7u32
    }
    else {
        return 0u32
    }
}

fn main() {
    let mut scores = HashMap::new();
    scores.insert('A', 1);
    scores.insert('B', 2);
    scores.insert('C', 3);
    scores.insert('X', 1);
    scores.insert('Y', 2);
    scores.insert('Z', 3);

    let file = File::open(Path::new("input.txt")).unwrap();

    let mut total = 0;

    for l in io::BufReader::new(file).lines() {
        if let Ok(ip) = l {
            let mut si = ip.split(" ");
            let them = si.next().unwrap();
            let me = si.next().unwrap();
            total += calc_score(them, me);
        }
    }

    println!("{total}")
}
