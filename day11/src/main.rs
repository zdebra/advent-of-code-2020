use array2d::Array2D;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("Hello, world from day 11!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut rows = vec![];
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        rows.push(chars);
    }

    let mut arr = Array2D::from_rows(&rows);
    print_arr(&arr);
    println!("");
    println!("");
    println!("");
    loop {
        let ops = run(&mut arr);
        // print_arr(&arr);
        // println!("");

        if ops == 0 {
            break;
        }
        // println!("ops: {}", ops);
    }

    println!("occupied seats: {}", occupied_seats(&arr));
}

fn run(arr: &mut Array2D<char>) -> usize {
    let mut ops = vec![];
    for row in 0..arr.column_len() {
        for col in 0..arr.row_len() {
            let element = arr.get(row, col).unwrap();
            let adjacent_cnt = adjacent_seats_count_part_2(&arr, row, col);
            // print!("{} ", adjacent_cnt);
            if *element == 'L' && adjacent_cnt == 0 {
                ops.push((row, col, '#'));
            } else if *element == '#' && adjacent_cnt > 4 {
                ops.push((row, col, 'L'));
            }
        }
        // println!("");
    }
    // println!("");

    for op in &ops {
        arr.set(op.0, op.1, op.2).unwrap();
    }
    ops.len()
}

fn occupied_seats(arr: &Array2D<char>) -> usize {
    arr.elements_row_major_iter().filter(|&&x| x == '#').count()
}

fn adjacent_seats_count(arr: &Array2D<char>, row: usize, column: usize) -> usize {
    let mut cnt = 0;

    let mut positions = vec![
        (row.checked_sub(1), Some(column)),
        (Some(row), column.checked_add(1)),
        (row.checked_add(1), Some(column)),
        (Some(row), column.checked_sub(1)),
        (row.checked_sub(1), column.checked_add(1)),
        (row.checked_add(1), column.checked_add(1)),
        (row.checked_add(1), column.checked_sub(1)),
        (row.checked_sub(1), column.checked_sub(1)),
    ];

    for pos in positions {
        let r = match pos.0 {
            Some(r) => r,
            None => continue,
        };
        let c = match pos.1 {
            Some(c) => c,
            None => continue,
        };
        if let Some(el) = arr.get(r, c) {
            if *el == '#' {
                cnt += 1;
            }
        }
    }
    cnt
}

fn adjacent_seats_count_part_2(arr: &Array2D<char>, row: usize, column: usize) -> usize {
    let mut cnt = 0;
    let curr = (row, column);
    let max_row = arr.column_len() - 1;
    let max_col = arr.row_len() - 1;

    let mut first_items = vec![];

    // down
    let mut beam = Beam {
        curr: curr,
        update_fn: Box::new(move |row: usize, col: usize| {
            if row > max_row {
                return None;
            }
            return Some((row + 1, col));
        }),
    };
    first_items.push(beam.first_item(arr));

    // top
    let mut beam = Beam {
        curr: curr,
        update_fn: Box::new(move |row: usize, col: usize| {
            if row == 0 {
                return None;
            }
            return Some((row - 1, col));
        }),
    };
    first_items.push(beam.first_item(arr));

    // left
    let mut beam = Beam {
        curr: curr,
        update_fn: Box::new(move |row: usize, col: usize| {
            if col == 0 {
                return None;
            }
            return Some((row, col - 1));
        }),
    };
    first_items.push(beam.first_item(arr));

    // right
    let mut beam = Beam {
        curr: curr,
        update_fn: Box::new(move |row: usize, col: usize| {
            if col > max_col {
                return None;
            }
            return Some((row, col + 1));
        }),
    };
    first_items.push(beam.first_item(arr));

    // top-right
    let mut beam = Beam {
        curr: curr,
        update_fn: Box::new(move |row: usize, col: usize| {
            if col > max_col || row == 0 {
                return None;
            }
            return Some((row - 1, col + 1));
        }),
    };
    first_items.push(beam.first_item(arr));

    // bottom-right
    let mut beam = Beam {
        curr: curr,
        update_fn: Box::new(move |row: usize, col: usize| {
            if col > max_col || row > max_row {
                return None;
            }
            return Some((row + 1, col + 1));
        }),
    };
    first_items.push(beam.first_item(arr));

    // bottom-left
    let mut beam = Beam {
        curr: curr,
        update_fn: Box::new(move |row: usize, col: usize| {
            if col == 0 || row > max_row {
                return None;
            }
            return Some((row + 1, col - 1));
        }),
    };
    first_items.push(beam.first_item(arr));

    // top-left
    let mut beam = Beam {
        curr: curr,
        update_fn: Box::new(move |row: usize, col: usize| {
            if col == 0 || row == 0 {
                return None;
            }
            return Some((row - 1, col - 1));
        }),
    };
    first_items.push(beam.first_item(arr));
    first_items.iter().filter(|&&x| x == '#').count()
}

fn print_arr(arr: &Array2D<char>) {
    for row in arr.as_rows() {
        for col in row {
            print!("{} ", col);
        }
        println!("");
    }
}

struct Beam<F> {
    curr: (usize, usize),
    update_fn: Box<F>,
}

impl<F> Iterator for Beam<F>
where
    F: Fn(usize, usize) -> Option<(usize, usize)>,
{
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        match (self.update_fn)(self.curr.0, self.curr.1) {
            Some(next) => {
                self.curr = next;
                Some(next)
            }
            None => None,
        }
    }
}

impl<F> Beam<F>
where
    F: Fn(usize, usize) -> Option<(usize, usize)>,
{
    fn first_item(&mut self, arr: &Array2D<char>) -> char {
        for place in self {
            if let Some(place) = arr.get(place.0, place.1) {
                return match place {
                    'L' => 'L',
                    '#' => '#',
                    _ => continue,
                };
            }
        }
        '.'
    }
}
