use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let file = File::open(Path::new("input.txt")).unwrap();

    let mut it = io::BufReader::new(file).lines().into_iter();

    let mut badges = vec![];

    while let Some(elf) = it.next() {
        let second_elf = &it.next().unwrap().unwrap();
        let third_elf = &it.next().unwrap().unwrap();
        for candidate in elf.unwrap().chars() {
            if second_elf.contains(candidate) {
                if third_elf.contains(candidate) {
                    badges.push(candidate);
                    break
                }
            }
        }
    }

    let mut total = 0;
    println!("{badges:?}");
    for c in badges {
        total += alphabet.iter().position(|&x| x == c).unwrap() + 1;
    }
    println!("{total}");
}
