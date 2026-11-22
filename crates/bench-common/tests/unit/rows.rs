use super::*;

#[test]
fn rows_are_distinct_and_deterministic() {
    assert_eq!(row(3), ("item-3".to_string(), 21));
    assert_ne!(row(1), row(2));
}
