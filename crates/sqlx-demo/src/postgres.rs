use bench_common::school::{School, SchoolInput};
use sqlx::{PgExecutor, PgPool, migrate::Migrator};
use uuid::Uuid;

use crate::{SchoolRow, school};

/// The SQL in `migrations/postgres`, compiled into the binary.
static MIGRATIONS: Migrator = sqlx::migrate!("./migrations/postgres");

pub async fn connect(url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPool::connect(url).await?;
    MIGRATIONS.run(&pool).await?;
    Ok(pool)
}

pub async fn create(db: impl PgExecutor<'_>, input: &SchoolInput) -> sqlx::Result<School> {
    let new = School::new(input);
    sqlx::query("INSERT INTO school (id, name, lon, lat, created_at) VALUES ($1, $2, $3, $4, $5)")
        .bind(new.id)
        .bind(&new.name)
        .bind(new.lon)
        .bind(new.lat)
        .bind(new.created_at)
        .execute(db)
        .await?;
    Ok(new)
}

pub async fn retrieve(db: impl PgExecutor<'_>, id: Uuid) -> sqlx::Result<Option<School>> {
    let row: Option<SchoolRow> =
        sqlx::query_as("SELECT id, name, lon, lat, created_at FROM school WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;
    Ok(row.map(school))
}

pub async fn update(
    db: impl PgExecutor<'_>,
    id: Uuid,
    input: &SchoolInput,
) -> sqlx::Result<Option<School>> {
    let row: Option<SchoolRow> = sqlx::query_as(
        "UPDATE school SET name = $2, lon = $3, lat = $4 WHERE id = $1 \
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

pub async fn delete(db: impl PgExecutor<'_>, id: Uuid) -> sqlx::Result<bool> {
    let deleted = sqlx::query("DELETE FROM school WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;
    Ok(deleted.rows_affected() > 0)
}

pub async fn page(db: impl PgExecutor<'_>, after: Uuid, size: i64) -> sqlx::Result<Vec<School>> {
    let rows: Vec<SchoolRow> = sqlx::query_as(
        "SELECT id, name, lon, lat, created_at FROM school WHERE id > $1 ORDER BY id LIMIT $2",
    )
    .bind(after)
    .bind(size)
    .fetch_all(db)
    .await?;
    Ok(rows.into_iter().map(school).collect())
}
