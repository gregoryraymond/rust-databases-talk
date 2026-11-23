use std::path::Path;

use bench_common::school::{School, SchoolInput};
use rusqlite::{Connection, OptionalExtension as _, Row, params};
use uuid::Uuid;

/// The SQL files in `migrations/`, in order. `PRAGMA user_version` counts how many have run.
const MIGRATIONS: &[&str] = &[include_str!("../migrations/0001_create_school.sql")];

pub fn open(path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    let applied: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    for (version, sql) in (1_i64..).zip(MIGRATIONS) {
        if version > applied {
            conn.execute_batch(sql)?;
            conn.pragma_update(None, "user_version", version)?;
        }
    }
    Ok(conn)
}

fn school(r: &Row<'_>) -> rusqlite::Result<School> {
    Ok(School {
        id: r.get(0)?,
        name: r.get(1)?,
        lon: r.get(2)?,
        lat: r.get(3)?,
        created_at: r.get(4)?,
    })
}

pub fn create(conn: &Connection, input: &SchoolInput) -> rusqlite::Result<School> {
    let new = School::new(input);
    conn.prepare_cached(
        "INSERT INTO school (id, name, lon, lat, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
    )?
    .execute(params![new.id, new.name, new.lon, new.lat, new.created_at])?;
    Ok(new)
}

pub fn retrieve(conn: &Connection, id: Uuid) -> rusqlite::Result<Option<School>> {
    conn.query_row(
        "SELECT id, name, lon, lat, created_at FROM school WHERE id = ?1",
        [id],
        school,
    )
    .optional()
}

pub fn update(
    conn: &Connection,
    id: Uuid,
    input: &SchoolInput,
) -> rusqlite::Result<Option<School>> {
    conn.query_row(
        "UPDATE school SET name = ?2, lon = ?3, lat = ?4 WHERE id = ?1 \
         RETURNING id, name, lon, lat, created_at",
        params![id, input.name, input.lon, input.lat],
        school,
    )
    .optional()
}

pub fn delete(conn: &Connection, id: Uuid) -> rusqlite::Result<bool> {
    Ok(conn.execute("DELETE FROM school WHERE id = ?1", [id])? > 0)
}
