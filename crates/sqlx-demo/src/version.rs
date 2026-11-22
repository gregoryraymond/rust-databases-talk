//! Reads the SQLite library version through sqlx.

use anyhow::{Context as _, Result};
use sqlx::{Connection as _, SqliteConnection};

use crate::connection::DATABASE_URL;

/// Returns the version of the SQLite library sqlx is linked against.
pub async fn library_version() -> Result<String> {
    let mut conn = SqliteConnection::connect(DATABASE_URL)
        .await
        .context("opening the demo database")?;
    sqlx::query_scalar("SELECT sqlite_version()")
        .fetch_one(&mut conn)
        .await
        .context("querying sqlite_version()")
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/version.rs");
}
