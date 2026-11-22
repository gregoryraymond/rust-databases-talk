//! The insert benchmark, through Diesel's typed `insert_into` DSL.

use std::time::Instant;

use anyhow::{Context as _, Result, ensure};
use bench_common::{InsertStats, database::fresh_database, rows::row, workload::CREATE_TABLE_SQL};
use diesel::prelude::*;

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

/// Inserts `rows` rows one statement at a time inside a single transaction, timing the inserts and commit.
pub fn insert_rows(rows: u32) -> Result<InsertStats> {
    let path = fresh_database(&format!("diesel-{rows}")).context("preparing the database file")?;
    let url = path.to_str().context("temp path is not UTF-8")?;
    let mut conn = SqliteConnection::establish(url).context("opening the database")?;
    diesel::sql_query(CREATE_TABLE_SQL)
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
        rows,
        elapsed,
    })
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/inserts.rs");
}
