use super::*;
use bench_common::workload::{ROW_COUNT, SMOKE_ROW_COUNT};

#[tokio::test]
async fn inserts_a_small_batch() {
    let stats = insert_rows(SMOKE_ROW_COUNT).await.unwrap();
    assert_eq!(stats.rows, SMOKE_ROW_COUNT);
    println!("{stats}");
}

#[tokio::test]
#[ignore = "3 million inserts; run with `cargo test --release -- --ignored --nocapture`"]
async fn three_million_inserts() {
    let stats = insert_rows(ROW_COUNT).await.unwrap();
    println!("{stats}");
}
