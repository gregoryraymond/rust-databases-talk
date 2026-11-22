//! The insert benchmark, through a SeaORM entity's `insert().exec()`.

use std::time::Instant;

use anyhow::{Context as _, Result, ensure};
use bench_common::{InsertStats, database::fresh_database, rows::row, workload::CREATE_TABLE_SQL};
use sea_orm::{
    ActiveValue::{NotSet, Set},
    ConnectionTrait as _, EntityTrait as _, PaginatorTrait as _, SqlxSqliteConnector,
    TransactionTrait as _,
    sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

/// The benchmark table as a SeaORM entity.
mod item {
    use sea_orm::entity::prelude::*;

    /// One row of the benchmark table.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "item")]
    pub struct Model {
        /// Rowid key.
        #[sea_orm(primary_key)]
        pub id: i64,
        /// Text column.
        pub name: String,
        /// Integer column.
        pub value: i64,
    }

    /// The table has no relations.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Inserts `rows` rows one statement at a time inside a single transaction, timing the inserts and commit.
pub async fn insert_rows(rows: u32) -> Result<InsertStats> {
    let path = fresh_database(&format!("seaorm-{rows}")).context("preparing the database file")?;
    let options = SqliteConnectOptions::new()
        .filename(&path)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .context("opening the database")?;
    let db = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);
    db.execute_unprepared(CREATE_TABLE_SQL)
        .await
        .context("creating the table")?;

    let start = Instant::now();
    let txn = db.begin().await.context("starting the transaction")?;
    for i in 0..rows {
        let (name, value) = row(i);
        let model = item::ActiveModel {
            id: NotSet,
            name: Set(name),
            value: Set(value),
        };
        item::Entity::insert(model)
            .exec(&txn)
            .await
            .context("inserting a row")?;
    }
    txn.commit().await.context("committing")?;
    let elapsed = start.elapsed();

    let count = item::Entity::find()
        .count(&db)
        .await
        .context("counting the rows")?;
    ensure!(
        count == u64::from(rows),
        "expected {rows} rows, found {count}"
    );
    Ok(InsertStats {
        library: "seaorm",
        rows,
        elapsed,
    })
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/inserts.rs");
}
