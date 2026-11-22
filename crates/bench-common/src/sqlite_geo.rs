//! The geospatial schema and queries on plain SQLite, without SpatiaLite.
//!
//! Points are `lon`/`lat` columns and outlines are WKT text with their bounding box beside them.
//! SQL narrows by box on an ordinary index; [`crate::spatial`] does the exact test in Rust.

/// Drops an earlier run's tables and creates fresh ones, one statement each.
pub const SETUP: &[&str] = &[
    "DROP TABLE IF EXISTS school",
    "DROP TABLE IF EXISTS suburb",
    "CREATE TABLE suburb (
        id BLOB PRIMARY KEY,
        name TEXT NOT NULL,
        state TEXT NOT NULL,
        outline TEXT NOT NULL,
        min_lon REAL NOT NULL,
        min_lat REAL NOT NULL,
        max_lon REAL NOT NULL,
        max_lat REAL NOT NULL,
        created_at TEXT NOT NULL
    )",
    "CREATE INDEX suburb_bounds_idx ON suburb (min_lon, max_lon)",
    "CREATE TABLE school (
        id BLOB PRIMARY KEY,
        name TEXT NOT NULL,
        sector TEXT NOT NULL,
        school_type TEXT NOT NULL,
        lon REAL NOT NULL,
        lat REAL NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL
    )",
    "CREATE INDEX school_lon_lat_idx ON school (lon, lat)",
];

/// Inserts a suburb: id, name, state, outline WKT, then its box west, south, east, north, then created at.
pub const INSERT_SUBURB: &str = "INSERT INTO suburb \
     (id, name, state, outline, min_lon, min_lat, max_lon, max_lat, created_at) \
     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)";

/// Inserts a school: id, name, sector, type, longitude, latitude, created at, updated at.
pub const INSERT_SCHOOL: &str = "INSERT INTO school \
     (id, name, sector, school_type, lon, lat, created_at, updated_at) \
     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)";

/// Reads one school back by id.
pub const SELECT_SCHOOL: &str = "SELECT id, name, lon, lat, created_at FROM school WHERE id = ?1";

/// Suburbs whose box holds a point: the candidates for the exact containment test.
pub const SUBURBS_AROUND: &str = "SELECT name, outline FROM suburb \
     WHERE ?1 BETWEEN min_lon AND max_lon AND ?2 BETWEEN min_lat AND max_lat ORDER BY name";

/// Schools inside a box west, east, south, north: the candidates for the exact distance test.
pub const SCHOOLS_IN_BOX: &str =
    "SELECT name, lon, lat FROM school WHERE lon BETWEEN ?1 AND ?2 AND lat BETWEEN ?3 AND ?4";

/// Moves a school: id, longitude, latitude, updated at.
pub const MOVE_SCHOOL: &str = "UPDATE school SET lon = ?2, lat = ?3, updated_at = ?4 WHERE id = ?1";

/// A school's location by id.
pub const SCHOOL_LOCATION: &str = "SELECT lon, lat FROM school WHERE id = ?1";

/// Deletes a school by id.
pub const DELETE_SCHOOL: &str = "DELETE FROM school WHERE id = ?1";

/// Counts the schools.
pub const COUNT_SCHOOLS: &str = "SELECT COUNT(*) FROM school";

/// Every column of one school by id.
pub const SELECT_SCHOOL_RECORD: &str = "SELECT id, name, sector, school_type, lon, lat, \
     created_at, updated_at FROM school WHERE id = ?1";

/// Every school, oldest first: UUIDv7 ids sort by creation time.
pub const LIST_SCHOOLS: &str = "SELECT id, name, sector, school_type, lon, lat, \
     created_at, updated_at FROM school ORDER BY id";

/// Replaces a school's fields: id, name, sector, type, longitude, latitude, updated at.
pub const UPDATE_SCHOOL: &str = "UPDATE school SET name = ?2, sector = ?3, school_type = ?4, \
     lon = ?5, lat = ?6, updated_at = ?7 WHERE id = ?1";

/// The school table, created only if missing, for the API, which keeps data between requests.
pub const CREATE_IF_MISSING: &[&str] = &[
    "CREATE TABLE IF NOT EXISTS school (
        id BLOB PRIMARY KEY,
        name TEXT NOT NULL,
        sector TEXT NOT NULL,
        school_type TEXT NOT NULL,
        lon REAL NOT NULL,
        lat REAL NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL
    )",
    "CREATE INDEX IF NOT EXISTS school_lon_lat_idx ON school (lon, lat)",
];
