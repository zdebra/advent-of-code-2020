use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("Hello, world from day 9!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut numbers = vec![];
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let num = line.parse::<usize>().unwrap();
        numbers.push(num);
    }
    numbers.sort();

    let mut diffs: [usize; 4] = [0; 4];
    let mut prev_num = 0;
    for n in &numbers {
        diffs[n - prev_num] += 1;
        prev_num = *n;
    }
    diffs[3] += 1;

    println!(
        "0: {}, 1: {}, 2: {}, 3: {}, res: {}",
        diffs[0],
        diffs[1],
        diffs[2],
        diffs[3],
        diffs[1] * diffs[3]
    );

    numbers.insert(0, 0);
    numbers.push(numbers.last().unwrap() + 3);
    let mut possibilities: Vec<usize> = vec![0; numbers.len()];
    possibilities[0] = 1;

    for i in 0..numbers.len() - 1 {
        let pivot = numbers[i];
        let pivot_possibilities = possibilities[i];

        for j in i + 1..numbers.len() {
            let diff = numbers[j] - pivot;
            if diff > 3 {
                break;
            }
            possibilities[j] += pivot_possibilities;
        }
    }

    println!("total possibilities: {}", possibilities.last().unwrap());
}
