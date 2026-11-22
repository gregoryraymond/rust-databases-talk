//! School CRUD through rusqlite: synchronous calls on one connection.

use std::path::Path;

use bench_common::{
    school::{SchoolInput, SchoolRecord},
    sqlite_geo,
};
use chrono::Utc;
use rusqlite::{Connection, OptionalExtension as _, Row, params};
use uuid::Uuid;

/// Opens the database at `path`, creating the school table if it is missing.
///
/// # Errors
///
/// Fails if the file cannot be opened or the table created.
pub fn open(path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    for sql in sqlite_geo::CREATE_IF_MISSING {
        conn.execute(sql, [])?;
    }
    Ok(conn)
}

/// A school from a row of [`sqlite_geo::SELECT_SCHOOL_RECORD`] or [`sqlite_geo::LIST_SCHOOLS`].
fn record(r: &Row<'_>) -> rusqlite::Result<SchoolRecord> {
    Ok(SchoolRecord {
        id: r.get(0)?,
        name: r.get(1)?,
        sector: r.get(2)?,
        school_type: r.get(3)?,
        lon: r.get(4)?,
        lat: r.get(5)?,
        created_at: r.get(6)?,
        updated_at: r.get(7)?,
    })
}

/// Inserts a school.
///
/// # Errors
///
/// Fails if the insert does.
pub fn create(conn: &Connection, input: &SchoolInput) -> rusqlite::Result<SchoolRecord> {
    let r = SchoolRecord::new(input);
    conn.execute(
        sqlite_geo::INSERT_SCHOOL,
        params![
            r.id,
            r.name,
            r.sector,
            r.school_type,
            r.lon,
            r.lat,
            r.created_at,
            r.updated_at
        ],
    )?;
    Ok(r)
}

/// One school by id.
///
/// # Errors
///
/// Fails if the query does.
pub fn get(conn: &Connection, id: Uuid) -> rusqlite::Result<Option<SchoolRecord>> {
    conn.query_row(sqlite_geo::SELECT_SCHOOL_RECORD, [id], record)
        .optional()
}

/// Every school, oldest first.
///
/// # Errors
///
/// Fails if the query does.
pub fn list(conn: &Connection) -> rusqlite::Result<Vec<SchoolRecord>> {
    conn.prepare(sqlite_geo::LIST_SCHOOLS)?
        .query_map([], record)?
        .collect()
}

/// Replaces a school's fields, returning the new row, or `None` if there is no such school.
///
/// # Errors
///
/// Fails if the update or the read back does.
pub fn update(
    conn: &Connection,
    id: Uuid,
    input: &SchoolInput,
) -> rusqlite::Result<Option<SchoolRecord>> {
    let updated = conn.execute(
        sqlite_geo::UPDATE_SCHOOL,
        params![
            id,
            input.name,
            input.sector,
            input.school_type,
            input.lon,
            input.lat,
            Utc::now()
        ],
    )?;
    if updated == 0 {
        return Ok(None);
    }
    get(conn, id)
}

/// Deletes a school, returning whether there was one.
///
/// # Errors
///
/// Fails if the delete does.
pub fn delete(conn: &Connection, id: Uuid) -> rusqlite::Result<bool> {
    Ok(conn.execute(sqlite_geo::DELETE_SCHOOL, [id])? > 0)
}
