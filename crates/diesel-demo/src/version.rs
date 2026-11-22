//! Reads the database version through Diesel.

use anyhow::{Context as _, Result};
use bench_common::{Backend, database::postgres_url};
use diesel::{pg::PgConnection, prelude::*};

use crate::connection::SQLITE_URL;

/// SQL functions Diesel does not ship, declared so queries using them type-check.
mod sql {
    use diesel::sql_types::Text;

    diesel::define_sql_function! {
        /// The SQLite `sqlite_version()` function.
        fn sqlite_version() -> Text;
    }

    diesel::define_sql_function! {
        /// The Postgres `version()` function.
        fn version() -> Text;
    }
}

/// Returns the SQLite library version or the Postgres server version.
pub fn library_version(backend: Backend) -> Result<String> {
    match backend {
        Backend::Sqlite => {
            let mut conn =
                SqliteConnection::establish(SQLITE_URL).context("opening the SQLite database")?;
            diesel::select(sql::sqlite_version())
                .get_result(&mut conn)
                .context("querying sqlite_version()")
        }
        Backend::Postgres => {
            let mut conn =
                PgConnection::establish(&postgres_url()).context("connecting to Postgres")?;
            diesel::select(sql::version())
                .get_result(&mut conn)
                .context("querying version()")
        }
    }
}
