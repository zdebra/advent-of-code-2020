use simple_error::SimpleError;
use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    println!("Hello, world from day 12!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut ship = Ship {
        x_pos: 0,
        y_pos: 0,
        x_pos_wp_rel: 10,
        y_pos_wp_rel: 1,
    };
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let ins = Instruction::from_str(&line).unwrap();
        // println!("ch: {}, val: {}", ins.ch, ins.val);
        ship.sail(ins);
        // println!(
        //     "ship({},{}), wp_rel({},{})",
        //     ship.x_pos, ship.y_pos, ship.x_pos_wp_rel, ship.y_pos_wp_rel
        // );
        // println!("");
    }
    println!(
        "manhattan distance: {}",
        ship.x_pos.abs() + ship.y_pos.abs()
    );
}

static DIRECTION_LEFT: &'static [char] = &['N', 'W', 'S', 'E'];
static DIRECTION_RIGHT: &'static [char] = &['N', 'E', 'S', 'W'];

struct Ship {
    x_pos: isize,
    y_pos: isize,
    x_pos_wp_rel: isize,
    y_pos_wp_rel: isize,
}

impl Ship {
    fn sail(&mut self, ins: Instruction) {
        match ins.ch {
            'N' => self.y_pos_wp_rel += ins.val,
            'S' => self.y_pos_wp_rel -= ins.val,
            'E' => self.x_pos_wp_rel += ins.val,
            'W' => self.x_pos_wp_rel -= ins.val,
            'L' => self.turn('L', usize::try_from(ins.val).unwrap()),
            'R' => self.turn('R', usize::try_from(ins.val).unwrap()),
            'F' => {
                self.x_pos += self.x_pos_wp_rel * ins.val;
                self.y_pos += self.y_pos_wp_rel * ins.val;
            }
            _ => panic!("unexpected command"),
        }
    }

    fn turn(&mut self, dir: char, angle: usize) {
        let turns = (angle % 360) / 90;
        let mut wp = (self.x_pos_wp_rel, self.y_pos_wp_rel);
        for i in 0..turns {
            wp = (wp.1, wp.0);
            if dir == 'L' {
                wp.0 *= -1;
            } else {
                wp.1 *= -1;
            }
        }
        self.x_pos_wp_rel = wp.0;
        self.y_pos_wp_rel = wp.1;
    }
}

struct Instruction {
    ch: char,
    val: isize,
}

impl FromStr for Instruction {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let ch = match chars.next() {
            Some(x) => x,
            None => return Err(SimpleError::new("expected first item to be a character")),
        };
        let rest: String = chars.collect();
        let val: isize = match rest.parse() {
            Ok(v) => v,
            Err(err) => return Err(SimpleError::with("expected rest to be a number", err)),
        };
        Ok(Self { ch: ch, val: val })
    }
}
