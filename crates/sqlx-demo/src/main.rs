//! sqlx on SQLite or Postgres: async SQL with compile-time checked queries and no DSL.
//!
//! Prints the database version, then times inserting `[rows]` rows
//! (default: [`ROW_COUNT`]) into a fresh table, on the backend chosen in `main`.
//! With `geo` as the argument, runs the geospatial example instead.

mod connection;
mod geospatial;
mod inserts;
mod version;

use std::{env, io::Write as _};

use anyhow::{Context as _, Result};
use bench_common::{Backend, geo_workload::GEO_COMMAND, workload::ROW_COUNT};

#[tokio::main]
async fn main() -> Result<()> {
    let backend = Backend::Sqlite;

    let arg = env::args().nth(1);
    let mut out = std::io::stdout();
    if arg.as_deref() == Some(GEO_COMMAND) {
        writeln!(out, "{}", geospatial::run().await?)?;
        return Ok(());
    }
    let rows = match arg {
        Some(arg) => arg.parse().context("rows must be a whole number")?,
        None => ROW_COUNT,
    };
    writeln!(
        out,
        "sqlx/{backend}: {}",
        version::library_version(backend).await?
    )?;
    writeln!(out, "{}", inserts::insert_rows(backend, rows).await?)?;
    Ok(())
}
