use std::time::Instant;

use anyhow::Result;
use bench_common::{
    Backend,
    database::{fresh_sqlite_path, postgres_url},
    school::{SchoolInput, rows_from_args},
};
use sea_orm::TransactionTrait as _;
use seaorm_demo::crud;

#[tokio::main]
async fn main() -> Result<()> {
    let backend = Backend::Sqlite;

    let url = match backend {
        Backend::Sqlite => format!(
            "sqlite://{}?mode=rwc",
            fresh_sqlite_path("seaorm")?.display()
        ),
        Backend::Postgres => postgres_url("talk_seaorm"),
    };
    let db = crud::connect(&url).await?;

    let school = crud::create(&db, &SchoolInput::example()).await?;
    println!("created   {school:?}");
    println!("retrieved {:?}", crud::retrieve(&db, school.id).await?);
    let moved = SchoolInput {
        lat: -37.8695,
        ..SchoolInput::example()
    };
    println!(
        "updated   {:?}",
        crud::update(&db, school.id, &moved).await?
    );
    println!("deleted   {}", crud::delete(&db, school.id).await?);

    let rows = rows_from_args()?;
    let start = Instant::now();
    let txn = db.begin().await?;
    for i in 0..rows {
        crud::create(&txn, &SchoolInput::numbered(i)).await?;
    }
    txn.commit().await?;
    println!(
        "seaorm/{backend:?}: {rows} creates in {:.2?}",
        start.elapsed()
    );
    Ok(())
}
