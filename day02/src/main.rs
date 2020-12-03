use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    println!("Hello, world from day 2!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut valid_cnt = 0;
    let mut valid_part_2_cnt = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let line = line.unwrap();
            let password = PasswordWithPolicy::from_str(&line.to_string()).unwrap();
            println!(
                "{} {} {} {} {} {}",
                password.ch,
                password.min,
                password.max,
                password.password,
                password.is_valid(),
                password.is_valid_part_two()
            );
            if password.is_valid() {
                valid_cnt = valid_cnt + 1;
            }
            if password.is_valid_part_two() {
                valid_part_2_cnt = valid_part_2_cnt + 1;
            }
        }
    }

    println!("valid password cnt: {}", valid_cnt);
    println!("valid password for part 2 cnt: {}", valid_part_2_cnt);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("open failed");
    Ok(io::BufReader::new(file).lines())
}

struct PasswordWithPolicy {
    ch: char,
    min: usize,
    max: usize,
    password: String,
}

impl FromStr for PasswordWithPolicy {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = line.split(' ').collect();
        assert!(splits.len() == 3);

        let counts: Vec<&str> = splits[0].split('-').collect();
        assert!(counts.len() == 2);

        let letter = splits[1]
            .chars()
            .nth(0)
            .expect("expected middle element to be `<char>:`");
        let password = splits[2];

        Ok(Self {
            ch: letter,
            min: counts[0]
                .parse::<usize>()
                .expect("expected counts to be in <u32>-<u32> format"),
            max: counts[1]
                .parse::<usize>()
                .expect("expected counts to be in <u32>-<u32> format"),
            password: String::from(password),
        })
    }
}

impl PasswordWithPolicy {
    fn is_valid(&self) -> bool {
        let occurences = self.password.chars().filter(|x| x == &self.ch).count();
        occurences >= self.min && occurences <= self.max
    }

    fn is_valid_part_two(&self) -> bool {
        let first_pos_char = self.password.chars().nth(self.min - 1);
        let second_pos_char = self.password.chars().nth(self.max - 1);

        if first_pos_char.is_some() && second_pos_char.is_some() {
            return (first_pos_char.unwrap() == self.ch && second_pos_char.unwrap() != self.ch)
                || (first_pos_char.unwrap() != self.ch && second_pos_char.unwrap() == self.ch);
        }
        if let Some(first_pos_char) = first_pos_char {
            return first_pos_char == self.ch;
        }
        if let Some(second_pos_char) = second_pos_char {
            return second_pos_char == self.ch;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_valid_part_two() {
        assert_eq!(
            PasswordWithPolicy {
                ch: 'a',
                min: 1,
                max: 3,
                password: "abcde".to_string(),
            }
            .is_valid_part_two(),
            true
        );
        assert_eq!(
            PasswordWithPolicy {
                ch: 'b',
                min: 1,
                max: 3,
                password: "cdefg".to_string(),
            }
            .is_valid_part_two(),
            false
        );
        assert_eq!(
            PasswordWithPolicy {
                ch: 'c',
                min: 2,
                max: 9,
                password: "ccccccccc".to_string(),
            }
            .is_valid_part_two(),
            false
        );
    }
}
