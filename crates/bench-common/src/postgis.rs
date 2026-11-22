//! The geospatial schema and queries on Postgres with PostGIS.
//!
//! Points bind as longitude then latitude. Distances go through `geography`, so they are metres.

/// Drops an earlier run's tables and creates fresh ones with spatial indexes, one statement each.
pub const SETUP: &[&str] = &[
    "CREATE EXTENSION IF NOT EXISTS postgis",
    "DROP TABLE IF EXISTS school",
    "DROP TABLE IF EXISTS suburb",
    "CREATE TABLE suburb (
        id UUID PRIMARY KEY,
        name TEXT NOT NULL,
        state TEXT NOT NULL,
        outline geometry(Polygon, 4326) NOT NULL,
        center geometry(Point, 4326) GENERATED ALWAYS AS (ST_Centroid(outline)) STORED,
        created_at TIMESTAMPTZ NOT NULL
    )",
    "CREATE INDEX suburb_outline_idx ON suburb USING GIST (outline)",
    "CREATE TABLE school (
        id UUID PRIMARY KEY,
        name TEXT NOT NULL,
        sector TEXT NOT NULL,
        school_type TEXT NOT NULL,
        location geometry(Point, 4326) NOT NULL,
        created_at TIMESTAMPTZ NOT NULL,
        updated_at TIMESTAMPTZ NOT NULL
    )",
    "CREATE INDEX school_location_idx ON school USING GIST (location)",
];

/// Inserts a suburb: id, name, state, outline as WKT, created at.
pub const INSERT_SUBURB: &str = "INSERT INTO suburb (id, name, state, outline, created_at) \
     VALUES ($1, $2, $3, ST_GeomFromText($4, 4326), $5)";

/// Inserts a school: id, name, sector, type, longitude, latitude, created and updated at.
pub const INSERT_SCHOOL: &str = "INSERT INTO school (id, name, sector, school_type, location, created_at, updated_at) \
     VALUES ($1, $2, $3, $4, ST_SetSRID(ST_MakePoint($5, $6), 4326), $7, $7)";

/// Reads one school back by id, with its point split into longitude and latitude.
pub const SELECT_SCHOOL: &str = "SELECT id, name, ST_X(location) AS lon, ST_Y(location) AS lat, \
     created_at FROM school WHERE id = $1";

/// The suburbs containing a point.
pub const SUBURB_AT: &str = "SELECT name FROM suburb \
     WHERE ST_Contains(outline, ST_SetSRID(ST_MakePoint($1, $2), 4326)) ORDER BY name";

/// Schools within a radius in metres of a point, nearest first.
pub const SCHOOLS_NEAR: &str = "SELECT name, \
     ST_Distance(location::geography, ST_MakePoint($1, $2)::geography) AS metres \
     FROM school \
     WHERE ST_DWithin(location::geography, ST_MakePoint($1, $2)::geography, $3) \
     ORDER BY metres";

/// Moves a school: id, longitude, latitude, updated at.
pub const MOVE_SCHOOL: &str = "UPDATE school \
     SET location = ST_SetSRID(ST_MakePoint($2, $3), 4326), updated_at = $4 WHERE id = $1";

/// The suburb a school sits in, by school id.
pub const SUBURB_OF_SCHOOL: &str = "SELECT su.name FROM school sc \
     JOIN suburb su ON ST_Contains(su.outline, sc.location) WHERE sc.id = $1";

/// Deletes a school by id.
pub const DELETE_SCHOOL: &str = "DELETE FROM school WHERE id = $1";

/// Counts the schools.
pub const COUNT_SCHOOLS: &str = "SELECT COUNT(*) FROM school";

/// Every column of one school by id, its point split into longitude and latitude.
pub const SELECT_SCHOOL_RECORD: &str = "SELECT id, name, sector, school_type, \
     ST_X(location) AS lon, ST_Y(location) AS lat, created_at, updated_at \
     FROM school WHERE id = $1";

/// Every school, oldest first: UUIDv7 ids sort by creation time.
pub const LIST_SCHOOLS: &str = "SELECT id, name, sector, school_type, \
     ST_X(location) AS lon, ST_Y(location) AS lat, created_at, updated_at \
     FROM school ORDER BY id";

/// Replaces a school's fields: id, name, sector, type, longitude, latitude, updated at.
pub const UPDATE_SCHOOL: &str = "UPDATE school SET name = $2, sector = $3, school_type = $4, \
     location = ST_SetSRID(ST_MakePoint($5, $6), 4326), updated_at = $7 WHERE id = $1";

/// The school table, created only if missing, for the API, which keeps data between requests.
pub const CREATE_IF_MISSING: &[&str] = &[
    "CREATE EXTENSION IF NOT EXISTS postgis",
    "CREATE TABLE IF NOT EXISTS school (
        id UUID PRIMARY KEY,
        name TEXT NOT NULL,
        sector TEXT NOT NULL,
        school_type TEXT NOT NULL,
        location geometry(Point, 4326) NOT NULL,
        created_at TIMESTAMPTZ NOT NULL,
        updated_at TIMESTAMPTZ NOT NULL
    )",
    "CREATE INDEX IF NOT EXISTS school_location_idx ON school USING GIST (location)",
];
