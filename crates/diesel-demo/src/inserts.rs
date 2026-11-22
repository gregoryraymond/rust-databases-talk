//! The insert benchmark, through Diesel's typed `insert_into` DSL.

use std::time::Instant;

use anyhow::{Context as _, Result, ensure};
use bench_common::{
    Backend, InsertStats,
    database::{fresh_database, postgres_url},
    rows::row,
    workload::{CREATE_TABLE_SQL, DROP_TABLE_SQL, POSTGRES_CREATE_TABLE_SQL},
};
use diesel::{pg::PgConnection, prelude::*};

diesel::table! {
    /// The benchmark table, as Diesel sees it.
    item (id) {
        /// Rowid key.
        id -> BigInt,
        /// Text column.
        name -> Text,
        /// Integer column.
        value -> BigInt,
    }
}

/// One row to insert; `id` is left to SQLite.
#[derive(Insertable)]
#[diesel(table_name = item)]
struct NewItem<'a> {
    /// Text column.
    name: &'a str,
    /// Integer column.
    value: i64,
}

/// A connection to either backend, so one code path drives both.
#[derive(diesel::MultiConnection)]
enum AnyConnection {
    /// A SQLite file.
    Sqlite(SqliteConnection),
    /// The Postgres server.
    Postgres(PgConnection),
}

/// Inserts `rows` rows one statement at a time inside a single transaction, timing the inserts and commit.
pub fn insert_rows(backend: Backend, rows: u32) -> Result<InsertStats> {
    let (mut conn, create_table) = match backend {
        Backend::Sqlite => {
            let path =
                fresh_database(&format!("diesel-{rows}")).context("preparing the database file")?;
            let url = path.to_str().context("temp path is not UTF-8")?;
            let conn = SqliteConnection::establish(url).context("opening the database")?;
            (AnyConnection::Sqlite(conn), CREATE_TABLE_SQL)
        }
        Backend::Postgres => {
            let conn =
                PgConnection::establish(&postgres_url()).context("connecting to Postgres")?;
            (AnyConnection::Postgres(conn), POSTGRES_CREATE_TABLE_SQL)
        }
    };
    diesel::sql_query(DROP_TABLE_SQL)
        .execute(&mut conn)
        .context("dropping the old table")?;
    diesel::sql_query(create_table)
        .execute(&mut conn)
        .context("creating the table")?;

    let start = Instant::now();
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        for i in 0..rows {
            let (name, value) = row(i);
            diesel::insert_into(item::table)
                .values(&NewItem { name: &name, value })
                .execute(conn)?;
        }
        Ok(())
    })
    .context("inserting the rows")?;
    let elapsed = start.elapsed();

    let count: i64 = item::table
        .count()
        .get_result(&mut conn)
        .context("counting the rows")?;
    ensure!(
        count == i64::from(rows),
        "expected {rows} rows, found {count}"
    );
    Ok(InsertStats {
        library: "diesel",
        backend,
        rows,
        elapsed,
    })
}
