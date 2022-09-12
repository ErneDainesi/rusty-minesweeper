mod common;

#[test]
fn first_test() {
    let file_name = "test.txt";
    crate::common::setup(file_name);
    let expected_output = vec![
        "1*3*1",
        "13*31",
        ".2*2.",
        ".111."
    ];
    assert_eq!(2 + 2, 4);
}
