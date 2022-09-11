use std::fs::File;
use std::io::{BufReader, BufRead, Result};

const FILE_NAME: &str = "./table.txt";
const MINE: u8 = b'*';

struct MineField<'a> {
    matrix: Vec<&'a [u8]>
}

struct Field {
    neighbours: Vec<u8>
}

impl Field {
    pub fn new(matrix: &MineField, row: usize, column: usize) -> Self {
        Self {
            neighbours: Self::get_neighbours(matrix, row, column)
        }
    }

    fn get_neighbours(mine_field: &MineField, row: usize, column: usize) -> Vec<u8> {
        let left_idx = safe_decrement(column, mine_field.matrix[row].len());
        let top_idx = safe_decrement(row, mine_field.matrix[row].len());
        let bottom_idx = row + 1;
        let right_idx = column + 1;
        let mut neighbours: Vec<u8> = vec![];
        let row_length = mine_field.matrix[row].len();
        if right_idx < row_length {
            neighbours.push(mine_field.matrix[row][right_idx]);
        }
        if left_idx < row_length {
            neighbours.push(mine_field.matrix[row][left_idx]);
        }
        if top_idx < row_length && right_idx < row_length {
            neighbours.push(mine_field.matrix[top_idx][right_idx]);
        }
        if top_idx < row_length && left_idx < row_length {
            neighbours.push(mine_field.matrix[top_idx][left_idx]);
        }
        if bottom_idx < mine_field.len() && left_idx < row_length {
            neighbours.push(mine_field.matrix[bottom_idx][left_idx]);
        }
        if top_idx < row_length {
            neighbours.push(mine_field.matrix[top_idx][column]);
        }
        if bottom_idx < mine_field.len() && right_idx < row_length {
            neighbours.push(mine_field.matrix[bottom_idx][right_idx]);
        }
        if bottom_idx < mine_field.len() {
            neighbours.push(mine_field.matrix[bottom_idx][column]);
        }
        neighbours
    }

    fn sweep_mines(&self, count: &mut i32) {
        for field in self.neighbours.iter() {
            if is_mine(*field) {
                *count += 1
            }
        }
    }
}

impl<'a> MineField<'a> {
    pub fn new() -> Self {
        let matrix: Vec<&[u8]> = vec![];
        Self {
           matrix
        }
    }

    pub fn push(&mut self, row: &'a [u8]) {
        self.matrix.push(row);
    }

    pub fn len(&self) -> usize {
        self.matrix.len()
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
    let mut mine_field = MineField::new();
    for line in &buf {
        mine_field.push(line.as_bytes())
    }
    let mut res = "".to_string();
    for i in 0..mine_field.len() {
        for j in 0..mine_field.matrix[i].len() {
            let mut counter = 0;
            if is_mine(mine_field.matrix[i][j]) {
                res += "*";
                continue;
            }
            let field: Field = Field::new(&mine_field, i, j);
            field.sweep_mines(&mut counter);
            if counter < 1 {
                res += &".".to_string();
            } else {
                res += &counter.to_string();
            }
        }
        println!("{:?}", res);
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
