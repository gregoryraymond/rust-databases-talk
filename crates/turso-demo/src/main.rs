//! Turso: an async, SQLite-compatible database engine rewritten in Rust.
//!
//! Prints the linked SQLite version, then times inserting `[rows]` rows
//! (default: [`ROW_COUNT`]) into a fresh database file.
//! With `geo` as the argument, runs the geospatial example instead.

mod connection;
mod geospatial;
mod inserts;
mod version;

use std::{env, io::Write as _};

use anyhow::{Context as _, Result};
use bench_common::{geo_workload::GEO_COMMAND, workload::ROW_COUNT};

#[tokio::main]
async fn main() -> Result<()> {
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
    writeln!(out, "turso: SQLite {}", version::library_version().await?)?;
    writeln!(out, "{}", inserts::insert_rows(rows).await?)?;
    Ok(())
}
