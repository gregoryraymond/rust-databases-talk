use bench_common::school::{School, SchoolInput};
use sqlx::{QueryBuilder, Sqlite, SqliteExecutor, SqlitePool, migrate::Migrator};
use uuid::Uuid;

use crate::{SchoolRow, school};

/// The SQL in `migrations/sqlite`, compiled into the binary.
static MIGRATIONS: Migrator = sqlx::migrate!("./migrations/sqlite");

pub async fn connect(url: &str) -> anyhow::Result<SqlitePool> {
    let pool = SqlitePool::connect(url).await?;
    MIGRATIONS.run(&pool).await?;
    Ok(pool)
}

pub async fn create(db: impl SqliteExecutor<'_>, input: &SchoolInput) -> sqlx::Result<School> {
    let new = School::new(input);
    sqlx::query("INSERT INTO school (id, name, lon, lat, created_at) VALUES (?1, ?2, ?3, ?4, ?5)")
        .bind(new.id)
        .bind(&new.name)
        .bind(new.lon)
        .bind(new.lat)
        .bind(new.created_at)
        .execute(db)
        .await?;
    Ok(new)
}

pub async fn retrieve(db: impl SqliteExecutor<'_>, id: Uuid) -> sqlx::Result<Option<School>> {
    let row: Option<SchoolRow> =
        sqlx::query_as("SELECT id, name, lon, lat, created_at FROM school WHERE id = ?1")
            .bind(id)
            .fetch_optional(db)
            .await?;
    Ok(row.map(school))
}

pub async fn update(
    db: impl SqliteExecutor<'_>,
    id: Uuid,
    input: &SchoolInput,
) -> sqlx::Result<Option<School>> {
    let row: Option<SchoolRow> = sqlx::query_as(
        "UPDATE school SET name = ?2, lon = ?3, lat = ?4 WHERE id = ?1 \
         RETURNING id, name, lon, lat, created_at",
    )
    .bind(id)
    .bind(&input.name)
    .bind(input.lon)
    .bind(input.lat)
    .fetch_optional(db)
    .await?;
    Ok(row.map(school))
}

pub async fn delete(db: impl SqliteExecutor<'_>, id: Uuid) -> sqlx::Result<bool> {
    let deleted = sqlx::query("DELETE FROM school WHERE id = ?1")
        .bind(id)
        .execute(db)
        .await?;
    Ok(deleted.rows_affected() > 0)
}

pub async fn create_many(
    db: impl SqliteExecutor<'_>,
    inputs: &[SchoolInput],
) -> sqlx::Result<Vec<School>> {
    let new: Vec<School> = inputs.iter().map(School::new).collect();
    let mut insert =
        QueryBuilder::<Sqlite>::new("INSERT INTO school (id, name, lon, lat, created_at) ");
    insert.push_values(&new, |mut row, s| {
        row.push_bind(s.id)
            .push_bind(&s.name)
            .push_bind(s.lon)
            .push_bind(s.lat)
            .push_bind(s.created_at);
    });
    insert.build().execute(db).await?;
    Ok(new)
}
