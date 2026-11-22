//! The insert benchmark, through sqlx's statement-cached `query().bind()`.

use std::time::Instant;

use anyhow::{Context as _, Result, ensure};
use bench_common::{
    InsertStats,
    database::fresh_database,
    rows::row,
    workload::{COUNT_SQL, CREATE_TABLE_SQL, INSERT_SQL},
};
use sqlx::{Connection as _, SqliteConnection, sqlite::SqliteConnectOptions};

/// Inserts `rows` rows one statement at a time inside a single transaction, timing the inserts and commit.
pub async fn insert_rows(rows: u32) -> Result<InsertStats> {
    let path = fresh_database(&format!("sqlx-{rows}")).context("preparing the database file")?;
    let options = SqliteConnectOptions::new()
        .filename(&path)
        .create_if_missing(true);
    let mut conn = SqliteConnection::connect_with(&options)
        .await
        .context("opening the database")?;
    sqlx::query(CREATE_TABLE_SQL)
        .execute(&mut conn)
        .await
        .context("creating the table")?;

    let start = Instant::now();
    let mut tx = conn.begin().await.context("starting the transaction")?;
    for i in 0..rows {
        let (name, value) = row(i);
        sqlx::query(INSERT_SQL)
            .bind(name)
            .bind(value)
            .execute(&mut *tx)
            .await
            .context("inserting a row")?;
    }
    tx.commit().await.context("committing")?;
    let elapsed = start.elapsed();

    let count: i64 = sqlx::query_scalar(COUNT_SQL)
        .fetch_one(&mut conn)
        .await
        .context("counting the rows")?;
    ensure!(
        count == i64::from(rows),
        "expected {rows} rows, found {count}"
    );
    Ok(InsertStats {
        library: "sqlx",
        rows,
        elapsed,
    })
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/inserts.rs");
}
