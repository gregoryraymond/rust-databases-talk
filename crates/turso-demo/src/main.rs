//! Turso: an async, SQLite-compatible database engine rewritten in Rust.
//!
//! Prints the linked SQLite version, then times inserting `[rows]` rows
//! (default: [`ROW_COUNT`]) into a fresh database file.

mod connection;
mod inserts;
mod version;

use std::{env, io::Write as _};

use anyhow::{Context as _, Result};
use bench_common::workload::ROW_COUNT;

#[tokio::main]
async fn main() -> Result<()> {
    let rows = match env::args().nth(1) {
        Some(arg) => arg.parse().context("rows must be a whole number")?,
        None => ROW_COUNT,
    };
    let mut out = std::io::stdout();
    writeln!(out, "turso: SQLite {}", version::library_version().await?)?;
    writeln!(out, "{}", inserts::insert_rows(rows).await?)?;
    Ok(())
}
