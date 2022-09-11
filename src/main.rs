use std::fs::File;
use std::io::{BufReader, BufRead, Result};

const FILE_NAME: &str = "./table.txt";
const MINE: u8 = b'*';

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

fn parse_lines(buf: Vec<String>) {
    let mut matrix = vec![];
    let mut output: Vec<String> = vec![];
    for line in &buf {
        matrix.push(line.as_bytes())
    }
    let mut res = "".to_string();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let mut counter = 0;
            if is_mine(matrix[i][j]) {
                res += "*";
                continue;
            }
            let left_idx = j.checked_sub(1).unwrap_or(matrix[i].len());
            let top_idx = i.checked_sub(1).unwrap_or(matrix[i].len());
            let bottom_idx = i + 1;
            let right_idx = j + 1;

            if right_idx < matrix[i].len() {
                let right = matrix[i][right_idx];
                if is_mine(right) {
                    counter += 1;
                }
            }

            if left_idx != matrix[i].len() {
                let left = matrix[i][left_idx];
                if is_mine(left) {
                    counter += 1;
                }
            }

            if top_idx != matrix[i].len() && right_idx < matrix[i].len() {
                let top_right = matrix[top_idx][right_idx];
                if is_mine(top_right) {
                    counter += 1;
                }
            }

            if top_idx != matrix[i].len() && left_idx != matrix[i].len() {
                let top_left = matrix[top_idx][left_idx];
                if is_mine(top_left) {
                    counter += 1;
                }
            }

            if bottom_idx < matrix.len() && left_idx != matrix[i].len() {
                let bottom_left = matrix[bottom_idx][left_idx];
                if is_mine(bottom_left) {
                    counter += 1;
                }
            }

            if top_idx < matrix[i].len() {
                let front = matrix[top_idx][j];
                if is_mine(front) {
                    counter += 1;
                }
            }

            if bottom_idx < matrix.len() && right_idx < matrix[i].len() {
                let bottom_right = matrix[bottom_idx][right_idx];
                if is_mine(bottom_right) {
                    counter += 1;
                }
            }

            if bottom_idx < matrix.len() {
                let back = matrix[bottom_idx][j];           
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
