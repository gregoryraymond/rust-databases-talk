//! The geospatial demo through rusqlite on plain SQLite.
//!
//! SpatiaLite would add geometry columns and `ST_*` functions, but loading an extension is
//! `unsafe` in rusqlite and this workspace forbids `unsafe`. So SQL narrows by bounding box
//! on ordinary indexes and Rust does the exact tests.

use anyhow::{Context as _, Result};
use bench_common::{
    database::fresh_database,
    geo_workload::{MOVE_TO, SEARCH_POINT, SEARCH_RADIUS_METRES},
    places::Bounds,
    report::{GeoReport, describe_school},
    sample_data::{SCHOOLS, SUBURBS},
    spatial::{nearest_within, outline_contains},
    sqlite_geo,
};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use uuid::Uuid;

/// Creates, reads, searches, moves and deletes the sample suburbs and schools in a fresh SQLite file.
pub fn run() -> Result<GeoReport> {
    let path = fresh_database("rusqlite-geo").context("preparing the database file")?;
    let mut conn = Connection::open(&path).context("opening the database")?;
    for sql in sqlite_geo::SETUP {
        conn.execute(sql, [])
            .with_context(|| format!("running {sql}"))?;
    }

    let now = Utc::now();
    let tx = conn.transaction().context("starting the transaction")?;
    for suburb in SUBURBS {
        let b = suburb.bounds();
        tx.execute(
            sqlite_geo::INSERT_SUBURB,
            params![
                Uuid::now_v7(),
                suburb.name,
                suburb.state,
                suburb.outline_wkt(),
                b.min_lon,
                b.min_lat,
                b.max_lon,
                b.max_lat,
                now,
            ],
        )
        .context("inserting a suburb")?;
    }
    let mut ids = Vec::with_capacity(SCHOOLS.len());
    for school in SCHOOLS {
        let id = Uuid::now_v7();
        tx.execute(
            sqlite_geo::INSERT_SCHOOL,
            params![
                id,
                school.name,
                school.sector,
                school.school_type,
                school.lon,
                school.lat,
                now,
                now,
            ],
        )
        .context("inserting a school")?;
        ids.push(id);
    }
    tx.commit().context("committing")?;
    let first = *ids.first().context("there are no sample schools")?;

    let (id, name, lon, lat, created_at): (Uuid, String, f64, f64, DateTime<Utc>) = conn
        .query_row(sqlite_geo::SELECT_SCHOOL, [first], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
        })
        .context("reading the school back")?;

    let suburbs_at_search_point = suburbs_at(&conn, SEARCH_POINT)?;

    let b = Bounds::around(SEARCH_POINT.0, SEARCH_POINT.1, SEARCH_RADIUS_METRES);
    let candidates = conn
        .prepare(sqlite_geo::SCHOOLS_IN_BOX)
        .context("preparing the box query")?
        .query_map(params![b.min_lon, b.max_lon, b.min_lat, b.max_lat], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })
        .context("finding schools in the box")?
        .collect::<rusqlite::Result<Vec<_>>>()
        .context("reading schools in the box")?;
    let nearby = nearest_within(SEARCH_POINT, SEARCH_RADIUS_METRES, candidates);

    conn.execute(
        sqlite_geo::MOVE_SCHOOL,
        params![first, MOVE_TO.0, MOVE_TO.1, Utc::now()],
    )
    .context("moving the school")?;
    let moved: (f64, f64) = conn
        .query_row(sqlite_geo::SCHOOL_LOCATION, [first], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })
        .context("reading the school's new location")?;
    let suburb_after_move = suburbs_at(&conn, moved)?.into_iter().next();

    conn.execute(sqlite_geo::DELETE_SCHOOL, [first])
        .context("deleting the school")?;
    let schools_after_delete = conn
        .query_row(sqlite_geo::COUNT_SCHOOLS, [], |r| r.get(0))
        .context("counting schools")?;

    Ok(GeoReport {
        library: "rusqlite",
        engine: "sqlite + geo",
        suburbs: SUBURBS.len(),
        schools: SCHOOLS.len(),
        read_back: describe_school(id, &name, lon, lat, created_at),
        suburbs_at_search_point,
        nearby,
        suburb_after_move,
        schools_after_delete,
    })
}

/// The suburbs containing `(lon, lat)`: a bounding-box query, then an exact test on each outline.
fn suburbs_at(conn: &Connection, (lon, lat): (f64, f64)) -> Result<Vec<String>> {
    let candidates = conn
        .prepare(sqlite_geo::SUBURBS_AROUND)
        .context("preparing the suburb query")?
        .query_map(params![lon, lat], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
        })
        .context("finding suburbs around the point")?
        .collect::<rusqlite::Result<Vec<_>>>()
        .context("reading suburbs around the point")?;
    let mut names = Vec::new();
    for (name, outline) in candidates {
        if outline_contains(&outline, lon, lat)? {
            names.push(name);
        }
    }
    Ok(names)
}
