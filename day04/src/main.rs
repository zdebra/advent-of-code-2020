use regex::Regex;
use simple_error::SimpleError;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    println!("Hello, world from day 4!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut valid_cnt = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let line = line.unwrap();
            // println!("LINE: {}", line);
            let passport = Passport::from_str(&line).unwrap();
            // println!("passport is valid: {}", passport.is_valid());
            if passport.is_valid() {
                valid_cnt += 1;
            }
        }
    }
    println!("valid passwords: {}", valid_cnt);
}

struct Passport {
    birth_year: String,      // byr (Birth Year)
    issue_year: String,      // iyr (Issue Year)
    expiration_year: String, // eyr (Expiration Year)
    height: String,          // hgt (Height)
    hair_color: String,      // hcl (Hair Color)
    eye_color: String,       // ecl (Eye Color)
    passport_id: String,     // pid (Passport ID)
    country_id: String,      // cid (Country ID)
}

impl Passport {
    fn has_all_required_fields(&self) -> bool {
        self.birth_year != ""
            && self.issue_year != ""
            && self.expiration_year != ""
            && self.height != ""
            && self.hair_color != ""
            && self.eye_color != ""
            && self.passport_id != ""
    }

    fn birth_year_valid(&self) -> bool {
        if self.birth_year.len() != 4 {
            return false;
        }
        let v = self.birth_year.parse::<u32>();
        if v.is_err() {
            return false;
        }
        let v = v.unwrap();
        v >= 1920 && v <= 2002
    }

    fn issue_year_valid(&self) -> bool {
        if self.issue_year.len() != 4 {
            return false;
        }
        let v = self.issue_year.parse::<u32>();
        if v.is_err() {
            return false;
        }
        let v = v.unwrap();
        v >= 2010 && v <= 2020
    }

    fn expiration_year_valid(&self) -> bool {
        if self.expiration_year.len() != 4 {
            return false;
        }
        let v = self.expiration_year.parse::<u32>();
        if v.is_err() {
            return false;
        }
        let v = v.unwrap();
        v >= 2020 && v <= 2030
    }

    fn height_valid(&self) -> bool {
        if self.height.ends_with("cm") {
            let v = self.height.trim_end_matches("cm");
            let v = v.parse::<u32>();
            if v.is_err() {
                return false;
            }
            let v = v.unwrap();
            return v >= 150 && v <= 193;
        }
        if self.height.ends_with("in") {
            let v = self.height.trim_end_matches("in");
            let v = v.parse::<u32>();
            if v.is_err() {
                return false;
            }
            let v = v.unwrap();
            return v >= 59 && v <= 76;
        }
        false
    }

    fn hair_color_valid(&self) -> bool {
        if self.hair_color.len() != 7 {
            return false;
        }
        let re = Regex::new("#[a-f0-9]{6}").unwrap();
        re.is_match(self.hair_color.as_str())
    }

    fn eye_color_valid(&self) -> bool {
        self.eye_color == "amb"
            || self.eye_color == "oth"
            || self.eye_color == "blu"
            || self.eye_color == "brn"
            || self.eye_color == "gry"
            || self.eye_color == "grn"
            || self.eye_color == "hzl"
    }

    fn passport_id_valid(&self) -> bool {
        if self.passport_id.len() != 9 {
            return false;
        }
        let re = Regex::new("[0-9]{9}").unwrap();
        re.is_match(self.passport_id.as_str())
    }

    fn is_valid(&self) -> bool {
        if !self.has_all_required_fields() {
            return false;
        }

        self.birth_year_valid()
            && self.issue_year_valid()
            && self.expiration_year_valid()
            && self.height_valid()
            && self.hair_color_valid()
            && self.eye_color_valid()
            && self.passport_id_valid()
    }
}

impl FromStr for Passport {
    type Err = SimpleError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport {
            birth_year: String::new(),
            issue_year: String::new(),
            expiration_year: String::new(),
            height: String::new(),
            hair_color: String::new(),
            eye_color: String::new(),
            passport_id: String::new(),
            country_id: String::new(),
        };
        let s = String::from_str(line).unwrap();
        let records: Vec<&str> = s.split(' ').collect();
        for record in records {
            if record.len() == 0 {
                continue;
            }
            let kv: Vec<&str> = record.split(':').collect();
            assert!(kv.len() == 2);
            match kv[0] {
                "byr" => passport.birth_year = kv[1].to_string(),
                "iyr" => passport.issue_year = kv[1].to_string(),
                "eyr" => passport.expiration_year = kv[1].to_string(),
                "hgt" => passport.height = kv[1].to_string(),
                "hcl" => passport.hair_color = kv[1].to_string(),
                "ecl" => passport.eye_color = kv[1].to_string(),
                "pid" => passport.passport_id = kv[1].to_string(),
                "cid" => passport.country_id = kv[1].to_string(),
                _ => return Err(SimpleError::new("invalid sequence")),
            }
        }
        Ok(passport)
    }
}

fn read_lines<P>(filename: P) -> io::Result<UntilDoubleNewline<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("open failed");
    let dl = UntilDoubleNewline {
        buf: io::BufReader::new(file),
    };
    Ok(dl)
}

/// UntilDoubleNewline reads stream until double new line characters
struct UntilDoubleNewline<B> {
    buf: B,
}

impl<B: BufRead> Iterator for UntilDoubleNewline<B> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        let mut buf = String::new();
        loop {
            let mut tmp = String::new();
            match self.buf.read_line(&mut tmp) {
                Ok(0) => {
                    if buf.len() > 0 {
                        return Some(Ok(buf));
                    }
                    return None;
                }
                Ok(_n) => {
                    if tmp.ends_with('\n') {
                        tmp.pop();
                        if tmp.ends_with('\r') {
                            tmp.pop();
                        }
                    }
                    if buf.len() > 0 {
                        buf.push(' ');
                    }
                    buf.push_str(tmp.as_str());
                    if tmp.len() == 0 {
                        return Some(Ok(buf));
                    }
                }
                Err(e) => return Some(Err(e)),
            };
        }
    }
}
