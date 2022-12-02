use std::cmp::Ordering;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::BinaryHeap;

#[derive(Eq)]
struct Elf {
    items: Vec<u32>,
    total: u32,
}

impl Ord for Elf {
    fn cmp(&self, other:&Self) -> Ordering {
        self.total.cmp(&other.total)
    }
}


impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.total == other.total
    }
}


fn main() {
    let mut elves = BinaryHeap::<Elf>::new();

    let ref mut load: Vec<u32> = vec!();

    let file = File::open(Path::new("input.txt")).unwrap();

    for l in io::BufReader::new(file).lines() {
        if let Ok(ip) = l {
            if ip != "" {
                load.push(ip.parse::<u32>().unwrap());
            }
            else {
                let elf = Elf {
                    items: load.to_owned(),
                    total: load.iter().sum::<u32>(),
                };
                load.clear();
                elves.push(elf);
            }
            
        }
        else {
            println!("meh")
        }
    }

    println!("{}", elves.pop().unwrap().items.iter().sum::<u32>())
}
