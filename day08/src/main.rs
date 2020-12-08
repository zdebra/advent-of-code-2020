use simple_error::SimpleError;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    println!("Hello, world from day 8!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut program = vec![];
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        program.push(Instruction::from_str(&line).expect("unexpected line format"))
    }

    for (index, ins) in program.iter().enumerate() {
        let mut tweaked_program = program.clone();
        match ins {
            Instruction::Jmp(arg) => {
                tweaked_program[index] = Instruction::Nop(*arg);
            }
            Instruction::Nop(arg) => {
                tweaked_program[index] = Instruction::Jmp(*arg);
            }
            _ => continue,
        }

        if let Ok(acc) = run(&tweaked_program) {
            println!("program success: acc value is {}", acc);
            break;
        } else {
            println!("program terminated with error");
        }
    }
}

fn run(program: &Vec<Instruction>) -> Result<isize, SimpleError> {
    let mut inst_pointer: isize = 0;
    let mut acc = 0;
    let mut imprints = vec![0; program.len()];
    loop {
        if inst_pointer as usize == program.len() {
            return Ok(acc);
        }
        if imprints[inst_pointer as usize] > 0 {
            return Err(SimpleError::new("infinite loop detected"));
        }
        imprints[inst_pointer as usize] += 1;
        let instruction_to_execute = &program[inst_pointer as usize];

        match instruction_to_execute {
            Instruction::Nop(_) => inst_pointer += 1,
            Instruction::Jmp(offset) => inst_pointer += offset,
            Instruction::Acc(val) => {
                inst_pointer += 1;
                acc += val
            }
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl FromStr for Instruction {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split_whitespace().collect();
        assert!(splits.len() == 2);

        let inst_arg = splits[1]
            .parse::<isize>()
            .expect("expected argument to be a number");

        match splits[0] {
            "nop" => Ok(Instruction::Nop(inst_arg)),
            "acc" => Ok(Instruction::Acc(inst_arg)),
            "jmp" => Ok(Instruction::Jmp(inst_arg)),
            _ => Err(SimpleError::new("invalid sequence")),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Acc(arg) => write!(f, "acc {}", arg),
            Instruction::Jmp(arg) => write!(f, "jmp {}", arg),
            Instruction::Nop(_) => write!(f, "nop"),
        }
    }
}
