//! The geospatial demo through sqlx on PostGIS: plain SQL, with points bound as longitude and latitude.

use anyhow::{Context as _, Result};
use bench_common::{
    database::postgres_url,
    geo_workload::{MOVE_TO, SEARCH_POINT, SEARCH_RADIUS_METRES},
    postgis,
    report::{GeoReport, Nearby, describe_school},
    sample_data::{SCHOOLS, SUBURBS},
};
use chrono::{DateTime, Utc};
use sqlx::{Connection as _, PgConnection};
use uuid::Uuid;

/// Creates, reads, searches, moves and deletes the sample suburbs and schools on PostGIS.
pub async fn run() -> Result<GeoReport> {
    let mut conn = PgConnection::connect(&postgres_url())
        .await
        .context("connecting to Postgres")?;
    for sql in postgis::SETUP {
        sqlx::query(*sql)
            .execute(&mut conn)
            .await
            .with_context(|| format!("running {sql}"))?;
    }

    let now = Utc::now();
    for suburb in SUBURBS {
        sqlx::query(postgis::INSERT_SUBURB)
            .bind(Uuid::now_v7())
            .bind(suburb.name)
            .bind(suburb.state)
            .bind(suburb.outline_wkt())
            .bind(now)
            .execute(&mut conn)
            .await
            .context("inserting a suburb")?;
    }
    let mut ids = Vec::with_capacity(SCHOOLS.len());
    for school in SCHOOLS {
        let id = Uuid::now_v7();
        sqlx::query(postgis::INSERT_SCHOOL)
            .bind(id)
            .bind(school.name)
            .bind(school.sector)
            .bind(school.school_type)
            .bind(school.lon)
            .bind(school.lat)
            .bind(now)
            .execute(&mut conn)
            .await
            .context("inserting a school")?;
        ids.push(id);
    }
    let first = *ids.first().context("there are no sample schools")?;

    let (id, name, lon, lat, created_at): (Uuid, String, f64, f64, DateTime<Utc>) =
        sqlx::query_as(postgis::SELECT_SCHOOL)
            .bind(first)
            .fetch_one(&mut conn)
            .await
            .context("reading the school back")?;

    let suburbs_at_search_point: Vec<String> = sqlx::query_scalar(postgis::SUBURB_AT)
        .bind(SEARCH_POINT.0)
        .bind(SEARCH_POINT.1)
        .fetch_all(&mut conn)
        .await
        .context("finding the suburb at the search point")?;

    let nearby: Vec<(String, f64)> = sqlx::query_as(postgis::SCHOOLS_NEAR)
        .bind(SEARCH_POINT.0)
        .bind(SEARCH_POINT.1)
        .bind(SEARCH_RADIUS_METRES)
        .fetch_all(&mut conn)
        .await
        .context("finding nearby schools")?;

    sqlx::query(postgis::MOVE_SCHOOL)
        .bind(first)
        .bind(MOVE_TO.0)
        .bind(MOVE_TO.1)
        .bind(Utc::now())
        .execute(&mut conn)
        .await
        .context("moving the school")?;
    let suburb_after_move: Option<String> = sqlx::query_scalar(postgis::SUBURB_OF_SCHOOL)
        .bind(first)
        .fetch_optional(&mut conn)
        .await
        .context("finding the school's new suburb")?;

    sqlx::query(postgis::DELETE_SCHOOL)
        .bind(first)
        .execute(&mut conn)
        .await
        .context("deleting the school")?;
    let schools_after_delete: i64 = sqlx::query_scalar(postgis::COUNT_SCHOOLS)
        .fetch_one(&mut conn)
        .await
        .context("counting schools")?;

    Ok(GeoReport {
        library: "sqlx",
        engine: "postgis",
        suburbs: SUBURBS.len(),
        schools: SCHOOLS.len(),
        read_back: describe_school(id, &name, lon, lat, created_at),
        suburbs_at_search_point,
        nearby: nearby
            .into_iter()
            .map(|(name, metres)| Nearby { name, metres })
            .collect(),
        suburb_after_move,
        schools_after_delete,
    })
}
