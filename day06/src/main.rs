use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world from day 6!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    if let Ok(lines) = read_lines(filename) {
        let mut group = String::new();
        let mut total = 0;
        for line in lines {
            let line = line.unwrap();
            if line == "" {
                let unique_chars = count_unique_chars(group);
                total += unique_chars;
                group = "".to_string();
            }
            group += &line;
        }
        total += count_unique_chars(group);
        println!("total is {}", total);
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    let mut people = 1;
    let mut uniques = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let line = line.unwrap();
            line.chars()
                .for_each(|ch| *counts.entry(ch).or_insert(0) += 1);
            if line == "" {
                for (key, val) in counts.iter() {
                    if *val == people - 1 {
                        uniques += 1;
                    }
                }
                counts = HashMap::new();
                people = 0;
                // uniques = 0;
            }
            people += 1;
        }

        for (key, val) in counts.iter() {
            if *val == people - 1 {
                uniques += 1;
            }
        }

        println!("uniques: {}", uniques);
    }
}

fn count_unique_chars(inp: String) -> usize {
    let mut unique_chars: Vec<char> = inp.chars().collect();
    unique_chars.sort();
    unique_chars.dedup();
    unique_chars.len()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("open failed");
    Ok(io::BufReader::new(file).lines())
}
