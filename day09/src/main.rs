use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("Hello, world from day 9!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let preamble = args[2].parse::<usize>().unwrap();

    let mut numbers = vec![];
    let mut index = 0;
    let mut step_1_num = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let num = line.parse::<usize>().unwrap();

        if index >= preamble && step_1_num == 0 {
            let len = numbers.len();
            if !testNumbers(&numbers[len - preamble..len], num) {
                step_1_num = num;
            }
        }
        numbers.push(num);
        index += 1;
    }

    println!("step 1 num: {}", step_1_num);
    println!("numbers len: {}", numbers.len());

    'outer: for i in 0..numbers.len() {
        let mut sum = 0;
        let mut smallest = numbers[i];
        let mut largest = numbers[i];
        for j in i..numbers.len() {
            sum += numbers[j];
            if numbers[j] > largest {
                largest = numbers[j];
            }

            if numbers[j] < smallest {
                smallest = numbers[j];
            }
            if sum == step_1_num {
                println!(
                    "got it smallest: {}, biggest: {}, sum: {}",
                    smallest,
                    largest,
                    smallest + largest
                );
                break 'outer;
            }
            if sum > step_1_num {
                break;
            }
        }
    }
}

fn testNumbers(numbers: &[usize], num: usize) -> bool {
    for i in 0..numbers.len() {
        for j in 1..numbers.len() {
            if numbers[i] + numbers[j] == num {
                return true;
            }
        }
    }
    false
}
