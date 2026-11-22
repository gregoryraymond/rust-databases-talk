//! Reads the SQLite library version through Diesel.

use anyhow::{Context as _, Result};
use diesel::prelude::*;

use crate::connection::DATABASE_URL;

/// SQL functions Diesel does not ship, declared so queries using them type-check.
mod sql {
    use diesel::sql_types::Text;

    diesel::define_sql_function! {
        /// SQLite's `sqlite_version()`.
        fn sqlite_version() -> Text;
    }
}

/// Returns the version of the SQLite library Diesel is linked against.
pub fn library_version() -> Result<String> {
    let mut conn =
        SqliteConnection::establish(DATABASE_URL).context("opening the demo database")?;
    diesel::select(sql::sqlite_version())
        .get_result(&mut conn)
        .context("querying sqlite_version()")
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/version.rs");
}
