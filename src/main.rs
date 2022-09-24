mod field;
mod file;
mod minefield;
use std::env;

/// Main function where we check if
/// the result of reading the file is an
/// error or is the valid string vector.
/// If the file is correctly read, the we
/// search for the mines in the `parse_lines` function.
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    match crate::file::read_file(file_name) {
        Ok(value) => {
            let output = crate::file::parse_lines(value);
            crate::file::print_output(output);
        }
        Err(err) => println!("[FILE READ ERROR] {:?}", err),
    }
}
