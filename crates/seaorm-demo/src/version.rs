//! Reads the database version through SeaORM.

use anyhow::{Context as _, Result};
use bench_common::{Backend, database::postgres_url};
use sea_orm::{ConnectionTrait as _, Database, Statement};

use crate::connection::SQLITE_URL;

/// Returns the SQLite library version or the Postgres server version.
pub async fn library_version(backend: Backend) -> Result<String> {
    let (url, sql) = match backend {
        Backend::Sqlite => (SQLITE_URL.to_owned(), "SELECT sqlite_version() AS version"),
        Backend::Postgres => (postgres_url(), "SELECT version() AS version"),
    };
    let db = Database::connect(url)
        .await
        .context("connecting to the database")?;
    let statement = Statement::from_string(db.get_database_backend(), sql);
    let row = db
        .query_one_raw(statement)
        .await
        .context("querying the version")?
        .context("the version query returned no row")?;
    row.try_get("", "version")
        .context("reading the version column")
}
