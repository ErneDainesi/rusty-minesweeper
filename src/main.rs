mod field;
mod minefield;
mod file;

/// Name of the file to read
const FILE_NAME: &str = "./table.txt";

/// Main function where we check if
/// the result of reading the file
/// is an error or is the valid string
/// vector
fn main() {
    match crate::file::read_file(FILE_NAME) {
        Ok(value) => {
           crate::file::parse_lines(value)
        },
        Err(err) => println!("[FILE READ ERROR] {:?}", err)
    }
}
