//! Reads the SQLite library version through rusqlite.

use anyhow::{Context as _, Result};
use rusqlite::Connection;

use crate::connection::DATABASE_URL;

/// Returns the version of the SQLite library rusqlite is linked against.
pub fn library_version() -> Result<String> {
    let conn = Connection::open(DATABASE_URL).context("opening the demo database")?;
    conn.query_row("SELECT sqlite_version()", [], |row| row.get(0))
        .context("querying sqlite_version()")
}
