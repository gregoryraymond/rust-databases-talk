use anyhow::Result;
use bench_common::{
    database::postgres_url,
    suburbs::{SEARCH_POINT, SUBURBS},
};
use sqlx_demo::postgres;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = postgres::connect(&postgres_url("talk_sqlx")).await?;

    sqlx::query("DELETE FROM suburb").execute(&pool).await?;
    for s in SUBURBS {
        sqlx::query("INSERT INTO suburb (name, outline) VALUES ($1, ST_GeomFromText($2, 4326))")
            .bind(s.name)
            .bind(s.outline_wkt())
            .execute(&pool)
            .await?;
    }

    let names: Vec<String> = sqlx::query_scalar(
        "SELECT name FROM suburb WHERE ST_Contains(outline, ST_SetSRID(ST_MakePoint($1, $2), 4326))",
    )
    .bind(SEARCH_POINT.0)
    .bind(SEARCH_POINT.1)
    .fetch_all(&pool)
    .await?;
    println!("sqlx: {SEARCH_POINT:?} is in {names:?}");
    Ok(())
}
