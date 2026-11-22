use super::*;
use bench_common::workload::{ROW_COUNT, SMOKE_ROW_COUNT};

#[test]
fn inserts_a_small_batch() {
    let stats = insert_rows(SMOKE_ROW_COUNT).unwrap();
    assert_eq!(stats.rows, SMOKE_ROW_COUNT);
    println!("{stats}");
}

#[test]
#[ignore = "3 million inserts; run with `cargo test --release -- --ignored --nocapture`"]
fn three_million_inserts() {
    let stats = insert_rows(ROW_COUNT).unwrap();
    println!("{stats}");
}
