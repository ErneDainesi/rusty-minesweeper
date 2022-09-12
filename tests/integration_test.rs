mod common;

#[test]
fn first_test() {
    crate::common::setup();
    assert_eq!(2 + 2, 4);
}
