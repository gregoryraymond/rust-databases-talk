//! School CRUD through diesel-async, over a deadpool pool: the same queries, awaited.

use bench_common::school::{SchoolInput, SchoolRecord};
use diesel::{OptionalExtension as _, QueryDsl as _, QueryResult, SelectableHelper as _};
use diesel_async::{
    AsyncPgConnection, RunQueryDsl as _,
    pooled_connection::{
        AsyncDieselConnectionManager,
        deadpool::{BuildError, Pool},
    },
};
use uuid::Uuid;

use crate::{
    models::{NewSchool, SchoolChanges, SchoolRow},
    schema::school,
};

/// A pool of async Postgres connections.
pub type AsyncPgPool = Pool<AsyncPgConnection>;

/// A pool for `url`. Connections are made when first needed.
///
/// # Errors
///
/// Fails if the pool cannot be built.
pub fn pool(url: &str) -> Result<AsyncPgPool, BuildError> {
    Pool::builder(AsyncDieselConnectionManager::<AsyncPgConnection>::new(url)).build()
}

/// Inserts a school.
///
/// # Errors
///
/// Fails if the insert does.
pub async fn create(
    conn: &mut AsyncPgConnection,
    input: &SchoolInput,
) -> QueryResult<SchoolRecord> {
    let record = SchoolRecord::new(input);
    diesel::insert_into(school::table)
        .values(NewSchool::from(&record))
        .execute(conn)
        .await?;
    Ok(record)
}

/// One school by id.
///
/// # Errors
///
/// Fails if the query does.
pub async fn get(conn: &mut AsyncPgConnection, id: Uuid) -> QueryResult<Option<SchoolRecord>> {
    let row = school::table
        .find(id)
        .select(SchoolRow::as_select())
        .first(conn)
        .await
        .optional()?;
    Ok(row.map(SchoolRecord::from))
}

/// Every school, oldest first.
///
/// # Errors
///
/// Fails if the query does.
pub async fn list(conn: &mut AsyncPgConnection) -> QueryResult<Vec<SchoolRecord>> {
    let rows = school::table
        .order(school::id)
        .select(SchoolRow::as_select())
        .load(conn)
        .await?;
    Ok(rows.into_iter().map(SchoolRecord::from).collect())
}

/// Replaces a school's fields, returning the new row, or `None` if there is no such school.
///
/// # Errors
///
/// Fails if the update does.
pub async fn update(
    conn: &mut AsyncPgConnection,
    id: Uuid,
    input: &SchoolInput,
) -> QueryResult<Option<SchoolRecord>> {
    let row = diesel::update(school::table.find(id))
        .set(SchoolChanges::from(input))
        .returning(SchoolRow::as_returning())
        .get_result(conn)
        .await
        .optional()?;
    Ok(row.map(SchoolRecord::from))
}

/// Deletes a school, returning whether there was one.
///
/// # Errors
///
/// Fails if the delete does.
pub async fn delete(conn: &mut AsyncPgConnection, id: Uuid) -> QueryResult<bool> {
    let deleted = diesel::delete(school::table.find(id)).execute(conn).await?;
    Ok(deleted > 0)
}
