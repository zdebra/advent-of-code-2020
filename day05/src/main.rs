use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world from day 5!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut max_seat_id = 0;
    if let Ok(lines) = read_lines(filename) {
        let mut seats = Vec::new();
        for line in lines {
            let line = line.unwrap();
            let row = interval_halving(line.get(0..7).unwrap().to_string(), (0, 127));
            let col = interval_halving(line.get(7..).unwrap().to_string(), (0, 7));
            let seat_id = row * 8 + col;
            seats.push(seat_id);
            if seat_id > max_seat_id {
                max_seat_id = seat_id;
            }
        }
        println!("max seat id is {}", max_seat_id);

        seats.sort();
        let mut prev = 0;
        for i in seats {
            if (i - 1) != prev {
                println!("my seat is {}", i - 1);
            }
            prev = i;
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("open failed");
    Ok(io::BufReader::new(file).lines())
}

fn interval_halving(inp: String, starting_bounds: (usize, usize)) -> usize {
    let bound = inp.chars().fold(starting_bounds, |acc, letter| {
        let delta = (acc.1 - acc.0) / 2 + 1;
        assert!(delta > 0);

        match letter {
            'F' | 'L' => (acc.0, acc.1 - delta),
            'B' | 'R' => (acc.0 + delta, acc.1),
            _ => panic!("unexpected letter"),
        }
    });
    assert!(bound.0 == bound.1);
    return bound.0;
}

#[test]
fn interval_halving_test() {
    assert_eq!(interval_halving("FBFBBFF".to_string(), (0, 127)), 44);
    assert_eq!(interval_halving("RLR".to_string(), (0, 7)), 5);
}
