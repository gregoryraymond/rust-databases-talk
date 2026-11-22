//! Where the API listens and how many connections it keeps.

/// The address the server binds to: every interface, so the dev container can forward it.
pub const BIND_ADDRESS: &str = "0.0.0.0:8080";

/// Connections each Postgres pool may open: plenty for a demo, well under Postgres' default 100.
pub const POOL_SIZE: u32 = 8;

/// The SQLite file rusqlite serves from, in the temp directory.
pub const RUSQLITE_DATABASE: &str = "api-rusqlite";

/// The database file Turso serves from, in the temp directory. Separate from rusqlite's, so
/// two engines never share one file.
pub const TURSO_DATABASE: &str = "api-turso";

/// The API's title in its OpenAPI document.
pub const API_TITLE: &str = "Schools";

/// The first argument that prints the OpenAPI document instead of starting the server.
pub const OPENAPI_COMMAND: &str = "openapi";
