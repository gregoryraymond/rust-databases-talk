//! School CRUD through sqlx: hand-written PostGIS SQL over a `PgPool`.

use bench_common::{
    postgis,
    school::{SchoolInput, SchoolRecord},
};
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

/// A school as a row, its point already split into longitude and latitude by the SQL.
#[derive(FromRow)]
struct SchoolRow {
    /// The id.
    id: Uuid,
    /// The school's name.
    name: String,
    /// Government, Catholic or Independent.
    sector: String,
    /// Primary, Secondary and so on.
    school_type: String,
    /// Longitude.
    lon: f64,
    /// Latitude.
    lat: f64,
    /// When the row was made.
    created_at: DateTime<Utc>,
    /// When the row last changed.
    updated_at: DateTime<Utc>,
}

impl From<SchoolRow> for SchoolRecord {
    fn from(r: SchoolRow) -> Self {
        Self {
            id: r.id,
            name: r.name,
            sector: r.sector,
            school_type: r.school_type,
            lon: r.lon,
            lat: r.lat,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

/// A pool of up to `max_connections` connections to `url`.
///
/// # Errors
///
/// Fails if the first connection cannot be made.
pub async fn pool(url: &str, max_connections: u32) -> sqlx::Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(url)
        .await
}

/// Inserts a school.
///
/// # Errors
///
/// Fails if the insert does.
pub async fn create(pool: &PgPool, input: &SchoolInput) -> sqlx::Result<SchoolRecord> {
    let record = SchoolRecord::new(input);
    sqlx::query(postgis::INSERT_SCHOOL)
        .bind(record.id)
        .bind(&record.name)
        .bind(&record.sector)
        .bind(&record.school_type)
        .bind(record.lon)
        .bind(record.lat)
        .bind(record.created_at)
        .execute(pool)
        .await?;
    Ok(record)
}

/// One school by id.
///
/// # Errors
///
/// Fails if the query does.
pub async fn get(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<SchoolRecord>> {
    let row: Option<SchoolRow> = sqlx::query_as(postgis::SELECT_SCHOOL_RECORD)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(SchoolRecord::from))
}

/// Every school, oldest first.
///
/// # Errors
///
/// Fails if the query does.
pub async fn list(pool: &PgPool) -> sqlx::Result<Vec<SchoolRecord>> {
    let rows: Vec<SchoolRow> = sqlx::query_as(postgis::LIST_SCHOOLS)
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(SchoolRecord::from).collect())
}

/// Replaces a school's fields, returning the new row, or `None` if there is no such school.
///
/// # Errors
///
/// Fails if the update or the read back does.
pub async fn update(
    pool: &PgPool,
    id: Uuid,
    input: &SchoolInput,
) -> sqlx::Result<Option<SchoolRecord>> {
    let updated = sqlx::query(postgis::UPDATE_SCHOOL)
        .bind(id)
        .bind(&input.name)
        .bind(&input.sector)
        .bind(&input.school_type)
        .bind(input.lon)
        .bind(input.lat)
        .bind(Utc::now())
        .execute(pool)
        .await?;
    if updated.rows_affected() == 0 {
        return Ok(None);
    }
    get(pool, id).await
}

/// Deletes a school, returning whether there was one.
///
/// # Errors
///
/// Fails if the delete does.
pub async fn delete(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
    let deleted = sqlx::query(postgis::DELETE_SCHOOL)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(deleted.rows_affected() > 0)
}
