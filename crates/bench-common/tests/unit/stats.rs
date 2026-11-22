use super::*;

#[test]
fn reports_rate_and_per_row_cost() {
    let stats = InsertStats {
        library: "test",
        backend: Backend::Sqlite,
        rows: 2_000,
        elapsed: Duration::from_secs(2),
    };
    assert!((stats.rows_per_second() - 1_000.0).abs() < f64::EPSILON);
    assert!((stats.micros_per_row() - 1_000.0).abs() < f64::EPSILON);
    assert_eq!(
        stats.to_string(),
        "test/sqlite: 2000 inserts in 2.00s (1000 rows/s, 1000.00 us/row)"
    );
}
