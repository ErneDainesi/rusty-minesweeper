mod common;
use buscaminas;

#[test]
fn test_search_for_mines_in_new_file() {
    let file_name = "test.txt";
    crate::common::setup(file_name);
    let expected_output = vec!["1*3*1", "13*31", ".2*2.", ".111."];
    let mut output: Vec<String> = vec![];
    match buscaminas::file::read_file(file_name) {
        Ok(value) => {
            output = buscaminas::file::parse_lines(value);
        }
        Err(error) => {
            print!("err {}", error);
        }
    }
    assert_eq!(&output, &expected_output);
}
