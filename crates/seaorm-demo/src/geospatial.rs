//! The geospatial demo through SeaORM on PostGIS.
//!
//! SeaORM has no geometry type, so anything touching a geometry column is a raw statement;
//! the entity covers the plain columns.

use anyhow::{Context as _, Result};
use bench_common::{
    database::postgres_url,
    geo_workload::{MOVE_TO, SEARCH_POINT, SEARCH_RADIUS_METRES},
    postgis,
    report::{GeoReport, Nearby, describe_school},
    sample_data::{SCHOOLS, SUBURBS},
};
use chrono::{DateTime, Utc};
use sea_orm::{
    ConnectionTrait as _, Database, DatabaseConnection, DbBackend, EntityTrait as _,
    FromQueryResult, PaginatorTrait as _, Statement, Value,
};
use seaorm_demo::school;
use uuid::Uuid;

/// A school read back with its point split into longitude and latitude.
#[derive(FromQueryResult)]
struct SchoolRow {
    /// The id.
    id: Uuid,
    /// The school's name.
    name: String,
    /// Longitude.
    lon: f64,
    /// Latitude.
    lat: f64,
    /// When the row was made.
    created_at: DateTime<Utc>,
}

/// A school near the search point.
#[derive(FromQueryResult)]
struct NearbyRow {
    /// The school's name.
    name: String,
    /// Metres from the search point.
    metres: f64,
}

/// A suburb's name.
#[derive(FromQueryResult)]
struct NameRow {
    /// The locality name.
    name: String,
}

/// A Postgres statement with its bound values.
fn statement(sql: &str, values: impl IntoIterator<Item = Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Postgres, sql, values)
}

/// Runs a statement that returns no rows.
async fn execute(
    db: &DatabaseConnection,
    sql: &str,
    values: impl IntoIterator<Item = Value>,
) -> Result<()> {
    db.execute_raw(statement(sql, values))
        .await
        .with_context(|| format!("running {sql}"))?;
    Ok(())
}

/// Creates, reads, searches, moves and deletes the sample suburbs and schools on PostGIS.
pub async fn run() -> Result<GeoReport> {
    let db = Database::connect(postgres_url())
        .await
        .context("connecting to Postgres")?;
    for sql in postgis::SETUP {
        db.execute_unprepared(sql)
            .await
            .with_context(|| format!("running {sql}"))?;
    }

    let first = insert_samples(&db).await?;

    let row = SchoolRow::find_by_statement(statement(postgis::SELECT_SCHOOL, [first.into()]))
        .one(&db)
        .await
        .context("reading the school back")?
        .context("the school was not there")?;

    let suburbs_at_search_point = NameRow::find_by_statement(statement(
        postgis::SUBURB_AT,
        [SEARCH_POINT.0.into(), SEARCH_POINT.1.into()],
    ))
    .all(&db)
    .await
    .context("finding the suburb at the search point")?
    .into_iter()
    .map(|r| r.name)
    .collect();

    let nearby = NearbyRow::find_by_statement(statement(
        postgis::SCHOOLS_NEAR,
        [
            SEARCH_POINT.0.into(),
            SEARCH_POINT.1.into(),
            SEARCH_RADIUS_METRES.into(),
        ],
    ))
    .all(&db)
    .await
    .context("finding nearby schools")?
    .into_iter()
    .map(|r| Nearby {
        name: r.name,
        metres: r.metres,
    })
    .collect();

    execute(
        &db,
        postgis::MOVE_SCHOOL,
        [
            first.into(),
            MOVE_TO.0.into(),
            MOVE_TO.1.into(),
            Utc::now().into(),
        ],
    )
    .await?;
    let suburb_after_move =
        NameRow::find_by_statement(statement(postgis::SUBURB_OF_SCHOOL, [first.into()]))
            .one(&db)
            .await
            .context("finding the school's new suburb")?
            .map(|r| r.name);

    school::Entity::delete_by_id(first)
        .exec(&db)
        .await
        .context("deleting the school")?;
    let remaining = school::Entity::find()
        .count(&db)
        .await
        .context("counting schools")?;

    Ok(GeoReport {
        library: "seaorm",
        engine: "postgis",
        suburbs: SUBURBS.len(),
        schools: SCHOOLS.len(),
        read_back: describe_school(row.id, &row.name, row.lon, row.lat, row.created_at),
        suburbs_at_search_point,
        nearby,
        suburb_after_move,
        schools_after_delete: i64::try_from(remaining).context("school count overflow")?,
    })
}

/// Inserts the sample suburbs and schools, returning the first school's id.
async fn insert_samples(db: &DatabaseConnection) -> Result<Uuid> {
    let now = Utc::now();
    for suburb in SUBURBS {
        execute(
            db,
            postgis::INSERT_SUBURB,
            [
                Uuid::now_v7().into(),
                suburb.name.into(),
                suburb.state.into(),
                suburb.outline_wkt().into(),
                now.into(),
            ],
        )
        .await?;
    }
    let mut ids = Vec::with_capacity(SCHOOLS.len());
    for school in SCHOOLS {
        let id = Uuid::now_v7();
        execute(
            db,
            postgis::INSERT_SCHOOL,
            [
                id.into(),
                school.name.into(),
                school.sector.into(),
                school.school_type.into(),
                school.lon.into(),
                school.lat.into(),
                now.into(),
            ],
        )
        .await?;
        ids.push(id);
    }
    ids.first().copied().context("there are no sample schools")
}
