use std::time::Instant;

use anyhow::Result;
use bench_common::{
    database::postgres_url,
    school::{SchoolInput, WRITERS, rows_from_args},
};
use diesel::QueryResult;
use diesel_async::{AsyncConnection as _, AsyncPgConnection};
use diesel_demo::{crud, crud_async};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<()> {
    let url = postgres_url("talk_diesel");
    crud::connect(&url)?;

    let rows = rows_from_args()?;
    let start = Instant::now();
    let mut writers = JoinSet::new();
    for writer in 0..WRITERS {
        let url = url.clone();
        writers.spawn(async move {
            let mut conn = AsyncPgConnection::establish(&url).await?;
            conn.transaction(async |conn| {
                for i in (writer..rows).step_by(WRITERS as usize) {
                    crud_async::create(conn, &SchoolInput::numbered(i)).await?;
                }
                QueryResult::Ok(())
            })
            .await?;
            anyhow::Ok(())
        });
    }
    while let Some(done) = writers.join_next().await {
        done??;
    }
    println!(
        "diesel-async/Postgres, {WRITERS} writers: {rows} creates in {:.2?}",
        start.elapsed()
    );
    Ok(())
}
