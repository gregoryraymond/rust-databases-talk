//! Reads the database version through sqlx.

use anyhow::{Context as _, Result};
use bench_common::{Backend, database::postgres_url};
use sqlx::{Connection as _, PgConnection, SqliteConnection};

use crate::connection::SQLITE_URL;

/// Returns the SQLite library version or the Postgres server version.
pub async fn library_version(backend: Backend) -> Result<String> {
    match backend {
        Backend::Sqlite => {
            let mut conn = SqliteConnection::connect(SQLITE_URL)
                .await
                .context("opening the SQLite database")?;
            sqlx::query_scalar("SELECT sqlite_version()")
                .fetch_one(&mut conn)
                .await
                .context("querying sqlite_version()")
        }
        Backend::Postgres => {
            let mut conn = PgConnection::connect(&postgres_url())
                .await
                .context("connecting to Postgres")?;
            sqlx::query_scalar("SELECT version()")
                .fetch_one(&mut conn)
                .await
                .context("querying version()")
        }
    }
}
