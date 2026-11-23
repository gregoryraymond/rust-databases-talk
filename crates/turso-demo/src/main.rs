use std::time::Instant;

use anyhow::{Context as _, Result};
use bench_common::{
    database::fresh_sqlite_path,
    school::{SchoolInput, rows_from_args},
};

#[tokio::main]
async fn main() -> Result<()> {
    let path = fresh_sqlite_path("turso")?;
    let db = turso_demo::open(path.to_str().context("path is not UTF-8")?).await?;
    let mut conn = db.connect()?;

    let school = turso_demo::create(&conn, &SchoolInput::example()).await?;
    println!("created   {school:?}");
    println!(
        "retrieved {:?}",
        turso_demo::retrieve(&conn, school.id).await?
    );
    let moved = SchoolInput {
        lat: -37.8695,
        ..SchoolInput::example()
    };
    println!(
        "updated   {:?}",
        turso_demo::update(&conn, school.id, &moved).await?
    );
    println!("deleted   {}", turso_demo::delete(&conn, school.id).await?);

    let rows = rows_from_args()?;
    let start = Instant::now();
    let tx = conn.transaction().await?;
    for i in 0..rows {
        turso_demo::create(&tx, &SchoolInput::numbered(i)).await?;
    }
    tx.commit().await?;
    println!("turso/Sqlite: {rows} creates in {:.2?}", start.elapsed());
    Ok(())
}
