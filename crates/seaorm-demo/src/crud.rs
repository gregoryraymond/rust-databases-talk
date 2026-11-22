//! School CRUD through SeaORM: the entity for plain columns, raw statements where geometry is involved.

use bench_common::{
    postgis,
    school::{SchoolInput, SchoolRecord},
};
use chrono::{DateTime, Utc};
use sea_orm::{
    ConnectOptions, ConnectionTrait as _, Database, DatabaseConnection, DbBackend, DbErr,
    EntityTrait as _, FromQueryResult, Statement,
};
use uuid::Uuid;

use crate::school;

/// A school read back with its point split into longitude and latitude.
#[derive(FromQueryResult)]
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

/// A connection pool of up to `max_connections` to `url`: SeaORM's connection is a sqlx pool.
///
/// # Errors
///
/// Fails if the first connection cannot be made.
pub async fn connect(url: &str, max_connections: u32) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(url);
    options.max_connections(max_connections);
    Database::connect(options).await
}

/// Inserts a school.
///
/// # Errors
///
/// Fails if the insert does.
pub async fn create(db: &DatabaseConnection, input: &SchoolInput) -> Result<SchoolRecord, DbErr> {
    let record = SchoolRecord::new(input);
    db.execute_raw(Statement::from_sql_and_values(
        DbBackend::Postgres,
        postgis::INSERT_SCHOOL,
        [
            record.id.into(),
            record.name.clone().into(),
            record.sector.clone().into(),
            record.school_type.clone().into(),
            record.lon.into(),
            record.lat.into(),
            record.created_at.into(),
        ],
    ))
    .await?;
    Ok(record)
}

/// One school by id.
///
/// # Errors
///
/// Fails if the query does.
pub async fn get(db: &DatabaseConnection, id: Uuid) -> Result<Option<SchoolRecord>, DbErr> {
    let row = SchoolRow::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        postgis::SELECT_SCHOOL_RECORD,
        [id.into()],
    ))
    .one(db)
    .await?;
    Ok(row.map(SchoolRecord::from))
}

/// Every school, oldest first.
///
/// # Errors
///
/// Fails if the query does.
pub async fn list(db: &DatabaseConnection) -> Result<Vec<SchoolRecord>, DbErr> {
    let rows = SchoolRow::find_by_statement(Statement::from_string(
        DbBackend::Postgres,
        postgis::LIST_SCHOOLS,
    ))
    .all(db)
    .await?;
    Ok(rows.into_iter().map(SchoolRecord::from).collect())
}

/// Replaces a school's fields, returning the new row, or `None` if there is no such school.
///
/// # Errors
///
/// Fails if the update or the read back does.
pub async fn update(
    db: &DatabaseConnection,
    id: Uuid,
    input: &SchoolInput,
) -> Result<Option<SchoolRecord>, DbErr> {
    let updated = db
        .execute_raw(Statement::from_sql_and_values(
            DbBackend::Postgres,
            postgis::UPDATE_SCHOOL,
            [
                id.into(),
                input.name.clone().into(),
                input.sector.clone().into(),
                input.school_type.clone().into(),
                input.lon.into(),
                input.lat.into(),
                Utc::now().into(),
            ],
        ))
        .await?;
    if updated.rows_affected() == 0 {
        return Ok(None);
    }
    get(db, id).await
}

/// Deletes a school through the entity, returning whether there was one.
///
/// # Errors
///
/// Fails if the delete does.
pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<bool, DbErr> {
    let deleted = school::Entity::delete_by_id(id).exec(db).await?;
    Ok(deleted.rows_affected > 0)
}
