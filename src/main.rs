use std::fs::File;
use std::io::{BufReader, BufRead, Result};

// 42 = *
// 46 = .

const FILE_NAME: &str = "./table.txt";

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
    value == b'*'
}

fn search_in_following_row(counter: &mut i32, matrix: &Vec<&[u8]>, row: &usize, j: &usize) {
    let next_row_front_value = matrix[*row][*j];
    let diagonal_next_right = matrix[*row][*j + 1];
    let diagonal_next_left = matrix[*row][j - 1];
    if is_mine(next_row_front_value) {
        *counter += 1;
    }
    if is_mine(diagonal_next_left) {
        *counter += 1;
    }
    if is_mine(diagonal_next_right) {
        *counter += 1;
    }
}

fn search_in_previous_row(counter: &mut i32, matrix: &Vec<&[u8]>, row: &usize, j: &usize) {
    let prev_row_front_value = matrix[*row][*j];
    let diagonal_prev_right = matrix[*row][*j + 1];
    let diagonal_prev_left = matrix[*row][*j - 1];
    if is_mine(prev_row_front_value) {
        *counter += 1;
    }
    if is_mine(diagonal_prev_left) {
        *counter += 1;
    }
    if is_mine(diagonal_prev_right) {
        *counter += 1;
    }
}

fn parse_lines(buf: Vec<String>) {
    let mut matrix = vec![];
    for line in &buf {
        matrix.push(line.as_bytes())
    }
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if is_mine(matrix[i][j]) {
                continue;
            }
            let mut counter = 0;
            let next_row = i + 1;
            let prev_row = i as i32 - 1;
            if next_row < matrix[i].len() {
                search_in_following_row(&mut counter, &matrix, &next_row, &j);
            }
            if prev_row < 0 {
                let prev_row = i - 1;
                search_in_previous_row(&mut counter, &matrix, &prev_row, &j);
            }

            println!("{}",counter);
        }
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
