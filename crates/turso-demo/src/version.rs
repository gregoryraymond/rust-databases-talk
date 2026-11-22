//! Reads the SQLite-compatible version Turso reports.

use anyhow::{Context as _, Result};
use turso::Builder;

use crate::connection::DATABASE_URL;

/// Returns the SQLite version Turso reports itself compatible with.
pub async fn library_version() -> Result<String> {
    let db = Builder::new_local(DATABASE_URL)
        .build()
        .await
        .context("opening the demo database")?;
    let conn = db.connect().context("connecting to the demo database")?;
    let mut rows = conn
        .query("SELECT sqlite_version()", ())
        .await
        .context("querying sqlite_version()")?;
    let row = rows
        .next()
        .await
        .context("reading the result row")?
        .context("sqlite_version() returned no row")?;
    row.get(0).context("reading the version column")
}
