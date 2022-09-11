use std::fs::File;
use std::io::{BufReader, BufRead, Result};

const FILE_NAME: &str = "./table.txt";
const MINE: u8 = b'*';

struct MineField<'a> {
    field: Vec<&'a [u8]>
}

struct Neighbour {
    value: usize,
    is_valid: bool
}

struct Neighbourhood {
    get_neighbours: Neighbour,
    left: Neighbour,
    front: Neighbour,
    right: Neighbour,
    top_left: Neighbour,
    top_right: Neighbour,
    bottom_left: Neighbour,
    bottom_right: Neighbour
}

impl Neighbourhood {
    // pub fn new(matrix: Vec<Vec<&u8>>, row: usize, column: usize) -> Self {
    //     // Self {
    //     //     is_valid: (),
    //     //     value: ()
    //     // }
    // }

    // fn check_if_row_is_valid() -> bool {
    //     if 
    // }

    fn get_neighbours(matrix: Vec<Vec<&u8>>, row: usize, column: usize) -> [usize; 4] {
        let left_idx = safe_decrement(column, matrix[row].len());
        let top_idx = safe_decrement(row, matrix[row].len());
        let bottom_idx = row + 1;
        let right_idx = column + 1;
        let idxs = [top_idx, right_idx, bottom_idx, left_idx];
        idxs
    }
}

impl<'a> MineField<'a> {
    pub fn new() -> Self {
        let field: Vec<&[u8]> = vec![];
        Self {
            field
        }
    }

    pub fn push(&mut self, row: &'a [u8]) {
        self.field.push(row);
    }

    pub fn len(&self) -> usize {
        self.field.len()
    }
}

fn read_file() -> Result<Vec<String>> {
    let file = File::open(FILE_NAME)?;
    let buffer_reader = BufReader::new(file);
    let mut v: Vec<String> = vec![];
    for line in buffer_reader.lines() {
        let value = line?;
        v.push(value);
    }
    Ok(v)
}

fn is_mine(value: u8) -> bool {
    value == MINE
}

fn safe_decrement(n: usize, default_value: usize) -> usize {
    n.checked_sub(1).unwrap_or(default_value)
}

fn parse_lines(buf: Vec<String>) {
    let mut matrix = MineField::new();
    let mut output: Vec<String> = vec![];
    for line in &buf {
        matrix.push(line.as_bytes())
    }
    let mut res = "".to_string();
    for i in 0..matrix.len() {
        for j in 0..matrix.field[i].len() {
            let mut counter = 0;
            if is_mine(matrix.field[i][j]) {
                res += "*";
                continue;
            }
            let left_idx = safe_decrement(j, matrix.field[i].len());
            let top_idx = safe_decrement(i, matrix.field[i].len());
            let bottom_idx = i + 1;
            let right_idx = j + 1;

            if right_idx < matrix.field[i].len() {
                let right = matrix.field[i][right_idx];
                if is_mine(right) {
                    counter += 1;
                }
            }

            if left_idx != matrix.field[i].len() {
                let left = matrix.field[i][left_idx];
                if is_mine(left) {
                    counter += 1;
                }
            }

            if top_idx != matrix.field[i].len() && right_idx < matrix.field[i].len() {
                let top_right = matrix.field[top_idx][right_idx];
                if is_mine(top_right) {
                    counter += 1;
                }
            }

            if top_idx != matrix.field[i].len() && left_idx != matrix.field[i].len() {
                let top_left = matrix.field[top_idx][left_idx];
                if is_mine(top_left) {
                    counter += 1;
                }
            }

            if bottom_idx < matrix.len() && left_idx != matrix.field[i].len() {
                let bottom_left = matrix.field[bottom_idx][left_idx];
                if is_mine(bottom_left) {
                    counter += 1;
                }
            }

            if top_idx < matrix.field[i].len() {
                let front = matrix.field[top_idx][j];
                if is_mine(front) {
                    counter += 1;
                }
            }

            if bottom_idx < matrix.len() && right_idx < matrix.field[i].len() {
                let bottom_right = matrix.field[bottom_idx][right_idx];
                if is_mine(bottom_right) {
                    counter += 1;
                }
            }

            if bottom_idx < matrix.len() {
                let back = matrix.field[bottom_idx][j];           
                if is_mine(back) {
                    counter += 1;
                }
            }

            if counter < 1 {
                res += &".".to_string();
            } else {
                res += &counter.to_string();
            }
        }

        println!("{:?}", res);
        output.push(res);
        res = "".to_string();
    }
}

fn main() {
    match read_file() {
        Ok(value) => {
            parse_lines(value)
        },
        Err(err) => println!("{:?}", err)
    }
}
