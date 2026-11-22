//! Where the Postgres server is.

/// Environment variable that overrides the Postgres URL.
pub const POSTGRES_URL_VAR: &str = "POSTGRES_URL";

/// The `postgres` service from the dev container's `docker-compose.yml`.
pub const DEFAULT_POSTGRES_URL: &str = "postgres://talk:talk@postgres:5432/talk";
