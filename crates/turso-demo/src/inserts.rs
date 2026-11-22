//! The insert benchmark, through one prepared Turso statement.

use std::time::Instant;

use anyhow::{Context as _, Result, ensure};
use bench_common::{
    Backend, InsertStats,
    database::fresh_database,
    rows::row,
    workload::{COUNT_SQL, CREATE_TABLE_SQL, INSERT_SQL},
};
use turso::Builder;

/// Inserts `rows` rows one statement at a time inside a single transaction, timing the inserts and commit.
pub async fn insert_rows(rows: u32) -> Result<InsertStats> {
    let path = fresh_database(&format!("turso-{rows}")).context("preparing the database file")?;
    let path = path.to_str().context("temp path is not UTF-8")?;
    let db = Builder::new_local(path)
        .build()
        .await
        .context("opening the database")?;
    let mut conn = db.connect().context("connecting to the database")?;
    conn.execute_batch(CREATE_TABLE_SQL)
        .await
        .context("creating the table")?;

    let start = Instant::now();
    let tx = conn
        .transaction()
        .await
        .context("starting the transaction")?;
    let mut stmt = tx
        .prepare(INSERT_SQL)
        .await
        .context("preparing the insert")?;
    for i in 0..rows {
        let (name, value) = row(i);
        stmt.execute((name, value))
            .await
            .context("inserting a row")?;
    }
    drop(stmt);
    tx.commit().await.context("committing")?;
    let elapsed = start.elapsed();

    let mut result = conn
        .query(COUNT_SQL, ())
        .await
        .context("counting the rows")?;
    let count: i64 = result
        .next()
        .await
        .context("reading the count")?
        .context("COUNT(*) returned no row")?
        .get(0)
        .context("reading the count column")?;
    ensure!(
        count == i64::from(rows),
        "expected {rows} rows, found {count}"
    );
    Ok(InsertStats {
        library: "turso",
        backend: Backend::Sqlite,
        rows,
        elapsed,
    })
}
