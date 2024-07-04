fn answer() -> i32 {
    26 + 16
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}
