use std::time::Instant;

use anyhow::Result;
use bench_common::{
    database::postgres_url,
    school::{SchoolInput, WRITERS, rows_from_args},
};
use sea_orm::TransactionTrait as _;
use seaorm_demo::crud;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<()> {
    let db = crud::connect(&postgres_url("talk_seaorm")).await?;

    let rows = rows_from_args()?;
    let start = Instant::now();
    let mut writers = JoinSet::new();
    for writer in 0..WRITERS {
        let db = db.clone();
        writers.spawn(async move {
            let txn = db.begin().await?;
            for i in (writer..rows).step_by(WRITERS as usize) {
                crud::create(&txn, &SchoolInput::numbered(i)).await?;
            }
            txn.commit().await?;
            anyhow::Ok(())
        });
    }
    while let Some(done) = writers.join_next().await {
        done??;
    }
    println!(
        "seaorm/Postgres, {WRITERS} writers: {rows} creates in {:.2?}",
        start.elapsed()
    );
    Ok(())
}
