use bench_common::school::{School, SchoolInput};
use diesel::{OptionalExtension as _, QueryDsl as _, QueryResult, SelectableHelper as _};
use diesel_async::{AsyncPgConnection, RunQueryDsl as _};
use uuid::Uuid;

use crate::{
    models::{SchoolChanges, SchoolRow},
    schema::school,
};

pub async fn create(conn: &mut AsyncPgConnection, input: &SchoolInput) -> QueryResult<School> {
    let new = School::new(input);
    diesel::insert_into(school::table)
        .values(SchoolRow::from(&new))
        .execute(conn)
        .await?;
    Ok(new)
}

pub async fn retrieve(conn: &mut AsyncPgConnection, id: Uuid) -> QueryResult<Option<School>> {
    let row = school::table
        .find(id.as_bytes().as_slice())
        .select(SchoolRow::as_select())
        .first(conn)
        .await
        .optional()?;
    Ok(row.map(School::from))
}

pub async fn update(
    conn: &mut AsyncPgConnection,
    id: Uuid,
    input: &SchoolInput,
) -> QueryResult<Option<School>> {
    diesel::update(school::table.find(id.as_bytes().as_slice()))
        .set(SchoolChanges::from(input))
        .execute(conn)
        .await?;
    retrieve(conn, id).await
}

pub async fn delete(conn: &mut AsyncPgConnection, id: Uuid) -> QueryResult<bool> {
    let deleted = diesel::delete(school::table.find(id.as_bytes().as_slice()))
        .execute(conn)
        .await?;
    Ok(deleted > 0)
}
