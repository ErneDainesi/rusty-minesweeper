use std::fs::File;
use std::io::prelude::*;

pub fn setup(file_name: &str) -> File {
    match create_file(file_name) {
        Ok(value) => value,
        Err(error) => {
            panic!("[FILE CREATION ERROR]: {}", error);
        }
    }
}

fn create_file(file_name: &str) -> std::io::Result<File> {
    let mut file = File::create(file_name)?;
    file.write_all(b".*.*.\n..*..\n..*..\n.....")?;
    Ok(file)
}
