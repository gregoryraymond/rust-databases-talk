use anyhow::Result;
use bench_common::{
    database::postgres_url,
    suburbs::{SEARCH_POINT, SUBURBS},
};
use sea_orm::{ConnectionTrait as _, DbBackend, Statement};
use seaorm_demo::crud;

#[tokio::main]
async fn main() -> Result<()> {
    let db = crud::connect(&postgres_url("talk_seaorm")).await?;

    db.execute_unprepared("DELETE FROM suburb").await?;
    for s in SUBURBS {
        db.execute_raw(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "INSERT INTO suburb (name, outline) VALUES ($1, ST_GeomFromText($2, 4326))",
            [s.name.into(), s.outline_wkt().into()],
        ))
        .await?;
    }

    let rows = db
        .query_all_raw(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT name FROM suburb WHERE ST_Contains(outline, ST_SetSRID(ST_MakePoint($1, $2), 4326))",
            [SEARCH_POINT.0.into(), SEARCH_POINT.1.into()],
        ))
        .await?;
    let names = rows
        .iter()
        .map(|row| row.try_get::<String>("", "name"))
        .collect::<Result<Vec<_>, _>>()?;
    println!("seaorm: {SEARCH_POINT:?} is in {names:?}");
    Ok(())
}
