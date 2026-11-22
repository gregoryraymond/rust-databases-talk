//! Where each benchmark run keeps its database.

use std::{env, ffi::OsString, fs, io, path::PathBuf};

use crate::connection::{DEFAULT_POSTGRES_URL, POSTGRES_URL_VAR};

/// Files SQLite may leave beside a database: the database itself, then its journals.
const DATABASE_FILE_SUFFIXES: [&str; 4] = ["", "-journal", "-wal", "-shm"];

/// Returns a path in the temp directory for database `name`, with any earlier run's files removed.
///
/// # Errors
///
/// Fails if an earlier run's file exists and cannot be removed.
pub fn fresh_database(name: &str) -> io::Result<PathBuf> {
    let path = env::temp_dir().join(format!("rust-databases-talk-{name}.db"));
    for suffix in DATABASE_FILE_SUFFIXES {
        let mut file = OsString::from(path.as_os_str());
        file.push(suffix);
        match fs::remove_file(&file) {
            Err(e) if e.kind() != io::ErrorKind::NotFound => return Err(e),
            _ => {}
        }
    }
    Ok(path)
}

/// The Postgres URL: [`POSTGRES_URL_VAR`] if set, otherwise [`DEFAULT_POSTGRES_URL`].
#[must_use]
pub fn postgres_url() -> String {
    env::var(POSTGRES_URL_VAR).unwrap_or_else(|_| DEFAULT_POSTGRES_URL.to_owned())
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/database.rs");
}
