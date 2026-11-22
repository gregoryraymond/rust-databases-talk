//! The geospatial demo through Turso.
//!
//! Turso has no SpatiaLite, so like rusqlite the SQL narrows by bounding box and Rust does the
//! exact tests. Turso has no UUID or date-time mapping either: ids go in as 16-byte blobs and
//! times as RFC 3339 text.

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
use turso::{Builder, Connection};
use turso_demo::crud::{id_blob, rows};
use uuid::Uuid;

/// Creates, reads, searches, moves and deletes the sample suburbs and schools in a fresh Turso file.
pub async fn run() -> Result<GeoReport> {
    let path = fresh_database("turso-geo").context("preparing the database file")?;
    let path = path.to_str().context("temp path is not UTF-8")?;
    let db = Builder::new_local(path)
        .build()
        .await
        .context("opening the database")?;
    let mut conn = db.connect().context("connecting to the database")?;
    for sql in sqlite_geo::SETUP {
        conn.execute(sql, ())
            .await
            .with_context(|| format!("running {sql}"))?;
    }

    let first = insert_samples(&mut conn).await?;

    let read = rows(&conn, sqlite_geo::SELECT_SCHOOL, [id_blob(first)]).await?;
    let row = read.first().context("the school was not there")?;
    let id = Uuid::from_slice(&row.get::<Vec<u8>>(0)?).context("reading the id")?;
    let name: String = row.get(1)?;
    let created_at: String = row.get(4)?;
    let created_at = DateTime::parse_from_rfc3339(&created_at)
        .context("reading created_at")?
        .with_timezone(&Utc);
    let read_back = describe_school(id, &name, row.get(2)?, row.get(3)?, created_at);

    let suburbs_at_search_point = suburbs_at(&conn, SEARCH_POINT).await?;

    let b = Bounds::around(SEARCH_POINT.0, SEARCH_POINT.1, SEARCH_RADIUS_METRES);
    let candidates = rows(
        &conn,
        sqlite_geo::SCHOOLS_IN_BOX,
        (b.min_lon, b.max_lon, b.min_lat, b.max_lat),
    )
    .await?
    .iter()
    .map(|r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
    .collect::<Result<Vec<_>>>()?;
    let nearby = nearest_within(SEARCH_POINT, SEARCH_RADIUS_METRES, candidates);

    conn.execute(
        sqlite_geo::MOVE_SCHOOL,
        (
            id_blob(first),
            MOVE_TO.0,
            MOVE_TO.1,
            Utc::now().to_rfc3339(),
        ),
    )
    .await
    .context("moving the school")?;
    let moved = rows(&conn, sqlite_geo::SCHOOL_LOCATION, [id_blob(first)]).await?;
    let moved = moved.first().context("the school was not there")?;
    let suburb_after_move = suburbs_at(&conn, (moved.get(0)?, moved.get(1)?))
        .await?
        .into_iter()
        .next();

    conn.execute(sqlite_geo::DELETE_SCHOOL, [id_blob(first)])
        .await
        .context("deleting the school")?;
    let count = rows(&conn, sqlite_geo::COUNT_SCHOOLS, ()).await?;
    let schools_after_delete = count.first().context("COUNT(*) returned no row")?.get(0)?;

    Ok(GeoReport {
        library: "turso",
        engine: "sqlite + geo",
        suburbs: SUBURBS.len(),
        schools: SCHOOLS.len(),
        read_back,
        suburbs_at_search_point,
        nearby,
        suburb_after_move,
        schools_after_delete,
    })
}

/// The suburbs containing `(lon, lat)`: a bounding-box query, then an exact test on each outline.
async fn suburbs_at(conn: &Connection, (lon, lat): (f64, f64)) -> Result<Vec<String>> {
    let mut names = Vec::new();
    for row in rows(conn, sqlite_geo::SUBURBS_AROUND, (lon, lat)).await? {
        let name: String = row.get(0)?;
        let outline: String = row.get(1)?;
        if outline_contains(&outline, lon, lat)? {
            names.push(name);
        }
    }
    Ok(names)
}

/// Inserts the sample suburbs and schools in one transaction, returning the first school's id.
async fn insert_samples(conn: &mut Connection) -> Result<Uuid> {
    let now = Utc::now().to_rfc3339();
    let tx = conn
        .transaction()
        .await
        .context("starting the transaction")?;
    for suburb in SUBURBS {
        let b = suburb.bounds();
        tx.execute(
            sqlite_geo::INSERT_SUBURB,
            (
                id_blob(Uuid::now_v7()),
                suburb.name,
                suburb.state,
                suburb.outline_wkt(),
                b.min_lon,
                b.min_lat,
                b.max_lon,
                b.max_lat,
                now.as_str(),
            ),
        )
        .await
        .context("inserting a suburb")?;
    }
    let mut ids = Vec::with_capacity(SCHOOLS.len());
    for school in SCHOOLS {
        let id = Uuid::now_v7();
        tx.execute(
            sqlite_geo::INSERT_SCHOOL,
            (
                id_blob(id),
                school.name,
                school.sector,
                school.school_type,
                school.lon,
                school.lat,
                now.as_str(),
                now.as_str(),
            ),
        )
        .await
        .context("inserting a school")?;
        ids.push(id);
    }
    tx.commit().await.context("committing")?;
    ids.first().copied().context("there are no sample schools")
}
