//! The insert benchmark, through sqlx's statement-cached `query().bind()`.

use std::time::{Duration, Instant};

use anyhow::{Context as _, Result, ensure};
use bench_common::{
    Backend, InsertStats,
    database::{fresh_database, postgres_url},
    rows::row,
    workload::{
        COUNT_SQL, CREATE_TABLE_SQL, DROP_TABLE_SQL, INSERT_SQL, POSTGRES_CREATE_TABLE_SQL,
        POSTGRES_INSERT_SQL,
    },
};
use sqlx::{Connection as _, PgConnection, SqliteConnection, sqlite::SqliteConnectOptions};

/// Inserts `rows` rows one statement at a time inside a single transaction, timing the inserts and commit.
pub async fn insert_rows(backend: Backend, rows: u32) -> Result<InsertStats> {
    let elapsed = match backend {
        Backend::Sqlite => insert_sqlite(rows).await?,
        Backend::Postgres => insert_postgres(rows).await?,
    };
    Ok(InsertStats {
        library: "sqlx",
        backend,
        rows,
        elapsed,
    })
}

/// The benchmark on a fresh SQLite file.
async fn insert_sqlite(rows: u32) -> Result<Duration> {
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
    Ok(elapsed)
}

/// The benchmark on a freshly created table on the Postgres server.
async fn insert_postgres(rows: u32) -> Result<Duration> {
    let mut conn = PgConnection::connect(&postgres_url())
        .await
        .context("connecting to Postgres")?;
    sqlx::query(DROP_TABLE_SQL)
        .execute(&mut conn)
        .await
        .context("dropping the old table")?;
    sqlx::query(POSTGRES_CREATE_TABLE_SQL)
        .execute(&mut conn)
        .await
        .context("creating the table")?;

    let start = Instant::now();
    let mut tx = conn.begin().await.context("starting the transaction")?;
    for i in 0..rows {
        let (name, value) = row(i);
        sqlx::query(POSTGRES_INSERT_SQL)
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
    Ok(elapsed)
}
