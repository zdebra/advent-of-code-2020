use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world from day one!");

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let numbers = file_to_vec(filename);
    'outer_two: for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                println!("{} + {} = 2020", numbers[i], numbers[j]);
                println!(
                    "{} * {} = {}",
                    numbers[i],
                    numbers[j],
                    numbers[i] * numbers[j]
                );
                break 'outer_two;
            }
        }
    }

    'outer_three: for i in 0..numbers.len() - 2 {
        for j in i + 1..numbers.len() - 1 {
            for k in j + 1..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!("{} + {} + {} = 2020", numbers[i], numbers[j], numbers[k]);
                    println!(
                        "{} * {} * {} = {}",
                        numbers[i],
                        numbers[j],
                        numbers[k],
                        numbers[i] * numbers[j] * numbers[k]
                    );
                    break 'outer_three;
                }
            }
        }
    }
}

fn file_to_vec<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let mut numbers = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let num = line.unwrap().parse::<i32>().unwrap();
            numbers.push(num);
        }
    }
    numbers
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
