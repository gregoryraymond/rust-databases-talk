use std::time::Instant;

use anyhow::Result;
use bench_common::{
    Backend,
    database::{fresh_sqlite_path, postgres_url},
    school::{SchoolInput, rows_from_args},
};
use sqlx_demo::{postgres, sqlite};

#[tokio::main]
async fn main() -> Result<()> {
    let backend = Backend::Sqlite;

    match backend {
        Backend::Sqlite => run_sqlite().await,
        Backend::Postgres => run_postgres().await,
    }
}

async fn run_sqlite() -> Result<()> {
    let path = fresh_sqlite_path("sqlx")?;
    let pool = sqlite::connect(&format!("sqlite://{}?mode=rwc", path.display())).await?;

    let school = sqlite::create(&pool, &SchoolInput::example()).await?;
    println!("created   {school:?}");
    println!("retrieved {:?}", sqlite::retrieve(&pool, school.id).await?);
    let moved = SchoolInput {
        lat: -37.8695,
        ..SchoolInput::example()
    };
    println!(
        "updated   {:?}",
        sqlite::update(&pool, school.id, &moved).await?
    );
    println!("deleted   {}", sqlite::delete(&pool, school.id).await?);

    let rows = rows_from_args()?;
    let start = Instant::now();
    let mut tx = pool.begin().await?;
    for i in 0..rows {
        sqlite::create(&mut *tx, &SchoolInput::numbered(i)).await?;
    }
    tx.commit().await?;
    println!("sqlx/Sqlite: {rows} creates in {:.2?}", start.elapsed());
    Ok(())
}

async fn run_postgres() -> Result<()> {
    let pool = postgres::connect(&postgres_url("talk_sqlx")).await?;

    let school = postgres::create(&pool, &SchoolInput::example()).await?;
    println!("created   {school:?}");
    println!(
        "retrieved {:?}",
        postgres::retrieve(&pool, school.id).await?
    );
    let moved = SchoolInput {
        lat: -37.8695,
        ..SchoolInput::example()
    };
    println!(
        "updated   {:?}",
        postgres::update(&pool, school.id, &moved).await?
    );
    println!("deleted   {}", postgres::delete(&pool, school.id).await?);

    let rows = rows_from_args()?;
    let start = Instant::now();
    let mut tx = pool.begin().await?;
    for i in 0..rows {
        postgres::create(&mut *tx, &SchoolInput::numbered(i)).await?;
    }
    tx.commit().await?;
    println!("sqlx/Postgres: {rows} creates in {:.2?}", start.elapsed());
    Ok(())
}
