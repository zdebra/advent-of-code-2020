use array2d::Array2D;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world from day 3!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut rows: Vec<Vec<SlopeElement>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            rows.push(
                line.unwrap()
                    .chars()
                    .map(|ch| SlopeElement::from(ch))
                    .collect(),
            );
        }
    }
    let slope = Slope {
        pattern: Array2D::from_rows(&rows),
    };

    let mut total = 1;
    for dir in vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
        total *= slope.count_trees(dir.0, dir.1);
    }

    println!("1-1 {}", slope.count_trees(1, 1));
    println!("1-3 {}", slope.count_trees(1, 3));
    println!("1-5 {}", slope.count_trees(1, 5));
    println!("1-7 {}", slope.count_trees(1, 7));
    println!("2-1 {}", slope.count_trees(2, 1));

    println!("trees encountered: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("open failed");
    Ok(io::BufReader::new(file).lines())
}

struct Slope {
    pattern: Array2D<SlopeElement>,
}

impl Slope {
    fn get(&self, row: usize, mut col: usize) -> Option<&SlopeElement> {
        if row > self.pattern.column_len() - 1 {
            return None;
        }

        if col > self.pattern.row_len() - 1 {
            col = col % self.pattern.row_len();
        }
        self.pattern.get(row, col)
    }

    fn count_trees(&self, row_index_delta: usize, column_index_delta: usize) -> usize {
        let mut row_index = 0;
        let mut column_index = 0;
        let mut tree_cnt = 0;
        loop {
            if let Some(element) = self.get(row_index, column_index) {
                if element == &SlopeElement::Tree {
                    tree_cnt += 1;
                }
            } else {
                break;
            }
            row_index += row_index_delta;
            column_index += column_index_delta;
        }
        tree_cnt
    }
}

#[derive(Clone, std::cmp::PartialEq)]
enum SlopeElement {
    Tree,
    OpenSquare,
}

impl From<char> for SlopeElement {
    fn from(ch: char) -> Self {
        match ch {
            '.' => SlopeElement::OpenSquare,
            '#' => SlopeElement::Tree,
            _ => panic!("invalid input char {}", ch),
        }
    }
}

impl fmt::Display for SlopeElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlopeElement::Tree => write!(f, "#"),
            SlopeElement::OpenSquare => write!(f, "."),
        }
    }
}
