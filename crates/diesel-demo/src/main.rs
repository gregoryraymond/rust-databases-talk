use std::time::Instant;

use anyhow::Result;
use bench_common::{
    Backend,
    database::{fresh_sqlite_path, postgres_url},
    school::{SchoolInput, rows_from_args},
};
use diesel::{Connection as _, QueryResult};
use diesel_demo::crud;

fn main() -> Result<()> {
    let backend = Backend::Sqlite;

    let url = match backend {
        Backend::Sqlite => fresh_sqlite_path("diesel")?.display().to_string(),
        Backend::Postgres => postgres_url("talk_diesel"),
    };
    let mut conn = crud::connect(&url)?;

    let school = crud::create(&mut conn, &SchoolInput::example())?;
    println!("created   {school:?}");
    println!("retrieved {:?}", crud::retrieve(&mut conn, school.id)?);
    let moved = SchoolInput {
        lat: -37.8695,
        ..SchoolInput::example()
    };
    println!(
        "updated   {:?}",
        crud::update(&mut conn, school.id, &moved)?
    );
    println!("deleted   {}", crud::delete(&mut conn, school.id)?);

    let rows = rows_from_args()?;
    let start = Instant::now();
    conn.transaction(|conn| {
        for i in 0..rows {
            crud::create(conn, &SchoolInput::numbered(i))?;
        }
        QueryResult::Ok(())
    })?;
    println!(
        "diesel/{backend:?}: {rows} creates in {:.2?}",
        start.elapsed()
    );
    Ok(())
}
