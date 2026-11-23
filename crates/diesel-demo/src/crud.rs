use bench_common::school::{School, SchoolInput};
use diesel::{pg::PgConnection, prelude::*, sqlite::SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness as _, embed_migrations};
use uuid::Uuid;

use crate::{
    models::{SchoolChanges, SchoolRow},
    schema::school,
};

/// The SQL in `migrations/postgres`, compiled into the binary.
const POSTGRES_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/postgres");

/// The SQL in `migrations/sqlite`, compiled into the binary.
const SQLITE_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/sqlite");

#[derive(diesel::MultiConnection)]
pub enum AnyConnection {
    Postgres(PgConnection),
    Sqlite(SqliteConnection),
}

pub fn connect(url: &str) -> anyhow::Result<AnyConnection> {
    let mut conn = AnyConnection::establish(url)?;
    let migrated = match &mut conn {
        AnyConnection::Postgres(c) => c.run_pending_migrations(POSTGRES_MIGRATIONS).map(|_| ()),
        AnyConnection::Sqlite(c) => c.run_pending_migrations(SQLITE_MIGRATIONS).map(|_| ()),
    };
    migrated.map_err(|e| anyhow::anyhow!(e))?;
    Ok(conn)
}

pub fn create(conn: &mut AnyConnection, input: &SchoolInput) -> QueryResult<School> {
    let new = School::new(input);
    diesel::insert_into(school::table)
        .values(SchoolRow::from(&new))
        .execute(conn)?;
    Ok(new)
}

pub fn retrieve(conn: &mut AnyConnection, id: Uuid) -> QueryResult<Option<School>> {
    let row = school::table
        .find(id.as_bytes().as_slice())
        .select(SchoolRow::as_select())
        .first(conn)
        .optional()?;
    Ok(row.map(School::from))
}

pub fn update(
    conn: &mut AnyConnection,
    id: Uuid,
    input: &SchoolInput,
) -> QueryResult<Option<School>> {
    diesel::update(school::table.find(id.as_bytes().as_slice()))
        .set(SchoolChanges::from(input))
        .execute(conn)?;
    retrieve(conn, id)
}

pub fn delete(conn: &mut AnyConnection, id: Uuid) -> QueryResult<bool> {
    let deleted = diesel::delete(school::table.find(id.as_bytes().as_slice())).execute(conn)?;
    Ok(deleted > 0)
}

pub fn page(conn: &mut AnyConnection, after: Uuid, size: i64) -> QueryResult<Vec<School>> {
    let rows = school::table
        .filter(school::id.gt(after.as_bytes().as_slice()))
        .order(school::id)
        .limit(size)
        .select(SchoolRow::as_select())
        .load(conn)?;
    Ok(rows.into_iter().map(School::from).collect())
}
