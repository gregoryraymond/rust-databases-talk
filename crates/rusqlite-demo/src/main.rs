//! rusqlite: an ergonomic synchronous wrapper over the SQLite C library.
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

fn main() -> Result<()> {
    let arg = env::args().nth(1);
    let mut out = std::io::stdout();
    if arg.as_deref() == Some(GEO_COMMAND) {
        writeln!(out, "{}", geospatial::run()?)?;
        return Ok(());
    }
    let rows = match arg {
        Some(arg) => arg.parse().context("rows must be a whole number")?,
        None => ROW_COUNT,
    };
    writeln!(out, "rusqlite: SQLite {}", version::library_version()?)?;
    writeln!(out, "{}", inserts::insert_rows(rows)?)?;
    Ok(())
}
