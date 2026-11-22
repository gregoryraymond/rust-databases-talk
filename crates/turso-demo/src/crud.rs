//! School CRUD through Turso: async calls, with UUIDs as blobs and times as RFC 3339 text.

use anyhow::{Context as _, Result};
use bench_common::{
    school::{SchoolInput, SchoolRecord},
    sqlite_geo,
};
use chrono::{DateTime, Utc};
use turso::{Builder, Connection, Database, IntoParams, Row};
use uuid::Uuid;

/// A UUID as the blob Turso stores.
#[must_use]
pub fn id_blob(id: Uuid) -> Vec<u8> {
    id.as_bytes().to_vec()
}

/// Every row a query returns.
///
/// # Errors
///
/// Fails if the query or reading a row does.
pub async fn rows(conn: &Connection, sql: &str, params: impl IntoParams) -> Result<Vec<Row>> {
    let mut result = conn
        .query(sql, params)
        .await
        .with_context(|| format!("running {sql}"))?;
    let mut rows = Vec::new();
    while let Some(row) = result.next().await.context("reading a row")? {
        rows.push(row);
    }
    Ok(rows)
}

/// Opens the database at `path`, creating the school table if it is missing.
///
/// # Errors
///
/// Fails if the file cannot be opened or the table created.
pub async fn open(path: &str) -> Result<Database> {
    let db = Builder::new_local(path)
        .build()
        .await
        .context("opening the database")?;
    let conn = db.connect().context("connecting")?;
    for sql in sqlite_geo::CREATE_IF_MISSING {
        conn.execute(sql, ())
            .await
            .with_context(|| format!("running {sql}"))?;
    }
    Ok(db)
}

/// A timestamp read back from RFC 3339 text.
fn timestamp(text: &str) -> Result<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(text)
        .context("reading a timestamp")?
        .with_timezone(&Utc))
}

/// A school from a row of [`sqlite_geo::SELECT_SCHOOL_RECORD`] or [`sqlite_geo::LIST_SCHOOLS`].
fn record(r: &Row) -> Result<SchoolRecord> {
    Ok(SchoolRecord {
        id: Uuid::from_slice(&r.get::<Vec<u8>>(0)?).context("reading the id")?,
        name: r.get(1)?,
        sector: r.get(2)?,
        school_type: r.get(3)?,
        lon: r.get(4)?,
        lat: r.get(5)?,
        created_at: timestamp(&r.get::<String>(6)?)?,
        updated_at: timestamp(&r.get::<String>(7)?)?,
    })
}

/// Inserts a school.
///
/// # Errors
///
/// Fails if the insert does.
pub async fn create(conn: &Connection, input: &SchoolInput) -> Result<SchoolRecord> {
    let r = SchoolRecord::new(input);
    conn.execute(
        sqlite_geo::INSERT_SCHOOL,
        (
            id_blob(r.id),
            r.name.as_str(),
            r.sector.as_str(),
            r.school_type.as_str(),
            r.lon,
            r.lat,
            r.created_at.to_rfc3339(),
            r.updated_at.to_rfc3339(),
        ),
    )
    .await
    .context("inserting a school")?;
    Ok(r)
}

/// One school by id.
///
/// # Errors
///
/// Fails if the query does.
pub async fn get(conn: &Connection, id: Uuid) -> Result<Option<SchoolRecord>> {
    let found = rows(conn, sqlite_geo::SELECT_SCHOOL_RECORD, [id_blob(id)]).await?;
    found.first().map(record).transpose()
}

/// Every school, oldest first.
///
/// # Errors
///
/// Fails if the query does.
pub async fn list(conn: &Connection) -> Result<Vec<SchoolRecord>> {
    rows(conn, sqlite_geo::LIST_SCHOOLS, ())
        .await?
        .iter()
        .map(record)
        .collect()
}

/// Replaces a school's fields, returning the new row, or `None` if there is no such school.
///
/// # Errors
///
/// Fails if the update or the read back does.
pub async fn update(
    conn: &Connection,
    id: Uuid,
    input: &SchoolInput,
) -> Result<Option<SchoolRecord>> {
    let updated = conn
        .execute(
            sqlite_geo::UPDATE_SCHOOL,
            (
                id_blob(id),
                input.name.as_str(),
                input.sector.as_str(),
                input.school_type.as_str(),
                input.lon,
                input.lat,
                Utc::now().to_rfc3339(),
            ),
        )
        .await
        .context("updating a school")?;
    if updated == 0 {
        return Ok(None);
    }
    get(conn, id).await
}

/// Deletes a school, returning whether there was one.
///
/// # Errors
///
/// Fails if the delete does.
pub async fn delete(conn: &Connection, id: Uuid) -> Result<bool> {
    let deleted = conn
        .execute(sqlite_geo::DELETE_SCHOOL, [id_blob(id)])
        .await
        .context("deleting a school")?;
    Ok(deleted > 0)
}
