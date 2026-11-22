//! School CRUD through synchronous Diesel, over an r2d2 pool.

use bench_common::school::{SchoolInput, SchoolRecord};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool, PoolError},
};
use uuid::Uuid;

use crate::{
    models::{NewSchool, SchoolChanges, SchoolRow},
    schema::school,
};

/// A pool of blocking Postgres connections.
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// A pool for `url`.
///
/// # Errors
///
/// Fails if the first connection cannot be made.
pub fn pool(url: &str) -> Result<PgPool, PoolError> {
    Pool::builder().build(ConnectionManager::new(url))
}

/// Inserts a school.
///
/// # Errors
///
/// Fails if the insert does.
pub fn create(conn: &mut PgConnection, input: &SchoolInput) -> QueryResult<SchoolRecord> {
    let record = SchoolRecord::new(input);
    diesel::insert_into(school::table)
        .values(NewSchool::from(&record))
        .execute(conn)?;
    Ok(record)
}

/// One school by id.
///
/// # Errors
///
/// Fails if the query does.
pub fn get(conn: &mut PgConnection, id: Uuid) -> QueryResult<Option<SchoolRecord>> {
    let row = school::table
        .find(id)
        .select(SchoolRow::as_select())
        .first(conn)
        .optional()?;
    Ok(row.map(SchoolRecord::from))
}

/// Every school, oldest first.
///
/// # Errors
///
/// Fails if the query does.
pub fn list(conn: &mut PgConnection) -> QueryResult<Vec<SchoolRecord>> {
    let rows = school::table
        .order(school::id)
        .select(SchoolRow::as_select())
        .load(conn)?;
    Ok(rows.into_iter().map(SchoolRecord::from).collect())
}

/// Replaces a school's fields, returning the new row, or `None` if there is no such school.
///
/// # Errors
///
/// Fails if the update does.
pub fn update(
    conn: &mut PgConnection,
    id: Uuid,
    input: &SchoolInput,
) -> QueryResult<Option<SchoolRecord>> {
    let row = diesel::update(school::table.find(id))
        .set(SchoolChanges::from(input))
        .returning(SchoolRow::as_returning())
        .get_result(conn)
        .optional()?;
    Ok(row.map(SchoolRecord::from))
}

/// Deletes a school, returning whether there was one.
///
/// # Errors
///
/// Fails if the delete does.
pub fn delete(conn: &mut PgConnection, id: Uuid) -> QueryResult<bool> {
    let deleted = diesel::delete(school::table.find(id)).execute(conn)?;
    Ok(deleted > 0)
}
