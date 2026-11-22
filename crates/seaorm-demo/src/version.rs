//! Reads the SQLite library version through SeaORM.

use anyhow::{Context as _, Result};
use sea_orm::{ConnectionTrait as _, Database, Statement};

use crate::connection::DATABASE_URL;

/// Returns the version of the SQLite library SeaORM is linked against.
pub async fn library_version() -> Result<String> {
    let db = Database::connect(DATABASE_URL)
        .await
        .context("opening the demo database")?;
    let statement = Statement::from_string(
        db.get_database_backend(),
        "SELECT sqlite_version() AS version",
    );
    let row = db
        .query_one_raw(statement)
        .await
        .context("querying sqlite_version()")?
        .context("sqlite_version() returned no row")?;
    row.try_get("", "version")
        .context("reading the version column")
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/version.rs");
}
