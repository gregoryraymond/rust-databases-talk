//! Diesel on SQLite: a compile-time checked query builder and ORM.
//!
//! Prints the linked SQLite version, then times inserting `[rows]` rows
//! (default: [`ROW_COUNT`]) into a fresh database file.

mod connection;
mod inserts;
mod version;

use std::{env, io::Write as _};

use anyhow::{Context as _, Result};
use bench_common::workload::ROW_COUNT;

fn main() -> Result<()> {
    let rows = match env::args().nth(1) {
        Some(arg) => arg.parse().context("rows must be a whole number")?,
        None => ROW_COUNT,
    };
    let mut out = std::io::stdout();
    writeln!(out, "diesel: SQLite {}", version::library_version()?)?;
    writeln!(out, "{}", inserts::insert_rows(rows)?)?;
    Ok(())
}
