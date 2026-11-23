use anyhow::Result;
use bench_common::school::{School, SchoolInput};
use chrono::DateTime;
use turso::{Builder, Connection, Database, Row};
use uuid::Uuid;

/// The SQL files in `migrations/`, in order. `PRAGMA user_version` counts how many have run.
const MIGRATIONS: &[&str] = &[include_str!("../migrations/0001_create_school.sql")];

pub async fn open(path: &str) -> Result<Database> {
    let db = Builder::new_local(path).build().await?;
    let conn = db.connect()?;
    let applied: i64 = {
        let mut rows = conn.query("PRAGMA user_version", ()).await?;
        match rows.next().await? {
            Some(row) => row.get(0)?,
            None => 0,
        }
    };
    for (version, sql) in (1_i64..).zip(MIGRATIONS) {
        if version > applied {
            conn.execute_batch(sql).await?;
            conn.execute(&format!("PRAGMA user_version = {version}"), ())
                .await?;
        }
    }
    Ok(db)
}

fn school(row: &Row) -> Result<School> {
    Ok(School {
        id: Uuid::from_slice(&row.get::<Vec<u8>>(0)?)?,
        name: row.get(1)?,
        lon: row.get(2)?,
        lat: row.get(3)?,
        created_at: DateTime::parse_from_rfc3339(&row.get::<String>(4)?)?.to_utc(),
    })
}

pub async fn create(conn: &Connection, input: &SchoolInput) -> Result<School> {
    let new = School::new(input);
    conn.execute(
        "INSERT INTO school (id, name, lon, lat, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            new.id.as_bytes().to_vec(),
            new.name.as_str(),
            new.lon,
            new.lat,
            new.created_at.to_rfc3339(),
        ),
    )
    .await?;
    Ok(new)
}

pub async fn retrieve(conn: &Connection, id: Uuid) -> Result<Option<School>> {
    let mut rows = conn
        .query(
            "SELECT id, name, lon, lat, created_at FROM school WHERE id = ?1",
            [id.as_bytes().to_vec()],
        )
        .await?;
    rows.next().await?.as_ref().map(school).transpose()
}

pub async fn update(conn: &Connection, id: Uuid, input: &SchoolInput) -> Result<Option<School>> {
    let mut rows = conn
        .query(
            "UPDATE school SET name = ?2, lon = ?3, lat = ?4 WHERE id = ?1 \
             RETURNING id, name, lon, lat, created_at",
            (
                id.as_bytes().to_vec(),
                input.name.as_str(),
                input.lon,
                input.lat,
            ),
        )
        .await?;
    rows.next().await?.as_ref().map(school).transpose()
}

pub async fn delete(conn: &Connection, id: Uuid) -> Result<bool> {
    let deleted = conn
        .execute("DELETE FROM school WHERE id = ?1", [id.as_bytes().to_vec()])
        .await?;
    Ok(deleted > 0)
}
