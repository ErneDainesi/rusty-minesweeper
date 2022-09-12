use std::fs::File;
use std::io::{BufReader, BufRead, Result};
mod field;
mod minefield;

/// Name of the file to read
const FILE_NAME: &str = "./table.txt";

/// Function responsable to read the file defined in
/// the const FILE_NAME.
/// This function iterates through each line and
/// builds a vector of Strings. The return value
/// is a Result.
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

/// This function receives the vector of Strings
/// created in the read_file function and
/// and each line is read as bytes.
/// A MineField is created which is used to
/// store the lines of the file. In each iteration
/// the minefield is sweeped and if a mine is found,
/// it will increment the mine counter and build
/// the output
fn parse_lines(buf: Vec<String>) {
    let mut mine_field = crate::minefield::MineField::new();
    for line in &buf {
        mine_field.push(line.as_bytes())
    }
    let mut res = "".to_string();
    for i in 0..mine_field.len() {
        for j in 0..mine_field.get_matrix()[i].len() {
            let mut counter = 0;
            if crate::field::is_mine(mine_field.get_matrix()[i][j]) {
                res += "*";
                continue;
            }
            let field = crate::field::Field::new(&mine_field, i, j);
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

/// Main function where we check if
/// the result of reading the file
/// is an error or is the valid string
/// vector
fn main() {
    match read_file() {
        Ok(value) => {
            parse_lines(value)
        },
        Err(err) => println!("[ERROR] {:?}", err)
    }
}
