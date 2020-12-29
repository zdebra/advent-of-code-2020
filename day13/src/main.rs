use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("Hello, world from day 13!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut lines = io::BufReader::new(file).lines();
    let guessed_ts: usize = lines.next().unwrap().unwrap().parse().unwrap();
    println!("guessed_ts: {}", guessed_ts);

    let mut timestamps = vec![];
    let mut min_id = usize::MAX;
    let mut min_diff = usize::MAX;

    let mut time_val = 0;
    let mut running_product = 1;

    for (index, split) in lines.next().unwrap().unwrap().split(",").enumerate() {
        if let Ok(bus_id) = split.parse::<usize>() {
            timestamps.push(bus_id);
            let nearest_ts = ((guessed_ts as f64) / (bus_id as f64)).ceil() * bus_id as f64;
            let diff = nearest_ts as usize - guessed_ts;
            if diff < min_diff {
                min_diff = diff;
                min_id = bus_id;
            }

            loop {
                if (time_val + index) % bus_id == 0 {
                    break;
                }
                time_val += running_product;
            }
            running_product *= bus_id;
        }
    }

    println!(
        "min diff: {}, min_id: {}, number: {}",
        min_diff,
        min_id,
        min_id * min_diff
    );
    println!("time_val: {}", time_val);
}
