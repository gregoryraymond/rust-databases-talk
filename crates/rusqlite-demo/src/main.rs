use std::time::Instant;

use anyhow::Result;
use bench_common::{
    database::fresh_sqlite_path,
    school::{SchoolInput, rows_from_args},
};

fn main() -> Result<()> {
    let mut conn = rusqlite_demo::open(&fresh_sqlite_path("rusqlite")?)?;

    let school = rusqlite_demo::create(&conn, &SchoolInput::example())?;
    println!("created   {school:?}");
    println!("retrieved {:?}", rusqlite_demo::retrieve(&conn, school.id)?);
    let moved = SchoolInput {
        lat: -37.8695,
        ..SchoolInput::example()
    };
    println!(
        "updated   {:?}",
        rusqlite_demo::update(&conn, school.id, &moved)?
    );
    println!("deleted   {}", rusqlite_demo::delete(&conn, school.id)?);

    let rows = rows_from_args()?;
    let start = Instant::now();
    let tx = conn.transaction()?;
    for i in 0..rows {
        rusqlite_demo::create(&tx, &SchoolInput::numbered(i))?;
    }
    tx.commit()?;
    println!("rusqlite/Sqlite: {rows} creates in {:.2?}", start.elapsed());
    Ok(())
}
