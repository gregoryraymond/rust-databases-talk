//! The insert benchmark, through one prepared rusqlite statement.

use std::time::Instant;

use anyhow::{Context as _, Result, ensure};
use bench_common::{
    Backend, InsertStats,
    database::fresh_database,
    rows::row,
    workload::{COUNT_SQL, CREATE_TABLE_SQL, INSERT_SQL},
};
use rusqlite::{Connection, params};

/// Inserts `rows` rows one statement at a time inside a single transaction, timing the inserts and commit.
pub fn insert_rows(rows: u32) -> Result<InsertStats> {
    let path =
        fresh_database(&format!("rusqlite-{rows}")).context("preparing the database file")?;
    let mut conn = Connection::open(&path).context("opening the database")?;
    conn.execute_batch(CREATE_TABLE_SQL)
        .context("creating the table")?;

    let start = Instant::now();
    let tx = conn.transaction().context("starting the transaction")?;
    {
        let mut stmt = tx.prepare(INSERT_SQL).context("preparing the insert")?;
        for i in 0..rows {
            let (name, value) = row(i);
            stmt.execute(params![name, value])
                .context("inserting a row")?;
        }
    }
    tx.commit().context("committing")?;
    let elapsed = start.elapsed();

    let count: i64 = conn
        .query_row(COUNT_SQL, [], |r| r.get(0))
        .context("counting the rows")?;
    ensure!(
        count == i64::from(rows),
        "expected {rows} rows, found {count}"
    );
    Ok(InsertStats {
        library: "rusqlite",
        backend: Backend::Sqlite,
        rows,
        elapsed,
    })
}
