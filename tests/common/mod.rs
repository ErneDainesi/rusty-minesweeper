use std::fs::File;
use std::io::prelude::*;

pub fn setup() -> File {
    match create_file() {
        Ok(value) => {
            value
        },
        Err(error) => {
            panic!("[FILE CREATION ERROR]: {}", error);
        }
    }
}

fn create_file() -> std::io::Result<File> {
    let mut file = File::create("test.txt")?;
    file.write_all(b".*.*.\n..*..\n..*..\n.....")?;
    Ok(file)
}
