use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;


fn main() {
    let mut seen = HashSet::<char>::new();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let file = File::open(Path::new("input.txt")).unwrap();

    let mut doubles = vec![];

    for l in io::BufReader::new(file).lines() {
        let ip = l.unwrap();
        let ipc = ip.clone();
        for (i, c) in ipc.into_bytes().iter().enumerate() {
            let half_length = ip.chars().count()/2;
            let cc = *c as char;
            if i >= half_length {
                if seen.contains(&cc) {
                    doubles.push(cc);
                    seen.clear();
                    continue;
                }
            }
            else {
                seen.insert(cc);
            }
        } 
    }
    let mut total = 0;
    for c in doubles {
        total += alphabet.iter().position(|&r| r == c).unwrap() + 1;
    }
    println!("{total}");
}
