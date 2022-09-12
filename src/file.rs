use std::fs::File;
use std::io::{BufRead, BufReader, Result};

/// Function responsable to read the file that is passed
/// as an argument.  
/// This function iterates through each line and
/// builds a vector of Strings. The return value
/// is a Result.
pub fn read_file(file_name: &str) -> Result<Vec<String>> {
    let file = File::open(file_name)?;
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
pub fn parse_lines(buf: Vec<String>) -> Vec<String> {
    let mut mine_field = crate::minefield::MineField::new();
    let mut output = vec![];
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
        output.push(res);
        res = "".to_string();
    }
    output
}
