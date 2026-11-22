//! Diesel on SQLite or Postgres: a compile-time checked query builder and ORM.
//!
//! Prints the database version, then times inserting `[rows]` rows
//! (default: [`ROW_COUNT`]) into a fresh table, on the backend chosen in `main`.

mod connection;
mod inserts;
mod version;

use std::{env, io::Write as _};

use anyhow::{Context as _, Result};
use bench_common::{Backend, workload::ROW_COUNT};

fn main() -> Result<()> {
    let backend = Backend::Sqlite;

    let rows = match env::args().nth(1) {
        Some(arg) => arg.parse().context("rows must be a whole number")?,
        None => ROW_COUNT,
    };
    let mut out = std::io::stdout();
    writeln!(
        out,
        "diesel/{backend}: {}",
        version::library_version(backend)?
    )?;
    writeln!(out, "{}", inserts::insert_rows(backend, rows)?)?;
    Ok(())
}
