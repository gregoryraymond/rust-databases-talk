use std::time::Instant;

use anyhow::Result;
use bench_common::{
    database::postgres_url,
    school::{SchoolInput, WRITERS, rows_from_args},
};
use sqlx_demo::postgres;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = postgres::connect(&postgres_url("talk_sqlx")).await?;

    let rows = rows_from_args()?;
    let start = Instant::now();
    let mut writers = JoinSet::new();
    for writer in 0..WRITERS {
        let pool = pool.clone();
        writers.spawn(async move {
            let mut tx = pool.begin().await?;
            for i in (writer..rows).step_by(WRITERS as usize) {
                postgres::create(&mut *tx, &SchoolInput::numbered(i)).await?;
            }
            tx.commit().await?;
            anyhow::Ok(())
        });
    }
    while let Some(done) = writers.join_next().await {
        done??;
    }
    println!(
        "sqlx/Postgres, {WRITERS} writers: {rows} creates in {:.2?}",
        start.elapsed()
    );
    Ok(())
}
