//! The size and shape of the insert benchmark.

/// Rows the full benchmark inserts: enough that per-row cost dominates setup.
pub const ROW_COUNT: u32 = 3_000_000;

/// Rows the smoke test inserts: enough to exercise the transaction, quick in a debug build.
pub const SMOKE_ROW_COUNT: u32 = 1_000;

/// The benchmark table: a rowid key plus one text and one integer column.
pub const CREATE_TABLE_SQL: &str =
    "CREATE TABLE item (id INTEGER PRIMARY KEY, name TEXT NOT NULL, value INTEGER NOT NULL)";

/// One row's insert, for the libraries that take raw SQL.
pub const INSERT_SQL: &str = "INSERT INTO item (name, value) VALUES (?1, ?2)";

/// Counts the rows afterwards, to check every insert landed.
pub const COUNT_SQL: &str = "SELECT COUNT(*) FROM item";
