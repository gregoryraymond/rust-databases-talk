//! Which database a demo runs against.

use std::fmt;

/// A database a demo can run against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// A SQLite database file.
    Sqlite,
    /// The Postgres server at [`postgres_url`](crate::database::postgres_url).
    Postgres,
}

impl Backend {
    /// Lower-case name, for output.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sqlite => "sqlite",
            Self::Postgres => "postgres",
        }
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/backend.rs");
}
