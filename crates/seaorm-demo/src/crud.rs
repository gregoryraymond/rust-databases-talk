use bench_common::school::{School, SchoolInput};
use sea_orm::{
    ActiveModelTrait as _, ActiveValue::Set, ColumnTrait as _, ConnectionTrait, Database,
    DatabaseConnection, DbErr, EntityTrait as _, QueryFilter as _, QueryOrder as _,
    QuerySelect as _,
};
use sea_orm_migration::MigratorTrait as _;
use uuid::Uuid;

use crate::{migrations::Migrator, school};

pub async fn connect(url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(url).await?;
    Migrator::up(&db, None).await?;
    Ok(db)
}

pub async fn create(db: &impl ConnectionTrait, input: &SchoolInput) -> Result<School, DbErr> {
    let new = School::new(input);
    let model = school::ActiveModel {
        id: Set(new.id),
        name: Set(new.name.clone()),
        lon: Set(new.lon),
        lat: Set(new.lat),
        created_at: Set(new.created_at),
    };
    school::Entity::insert(model).exec(db).await?;
    Ok(new)
}

pub async fn retrieve(db: &impl ConnectionTrait, id: Uuid) -> Result<Option<School>, DbErr> {
    let model = school::Entity::find_by_id(id).one(db).await?;
    Ok(model.map(School::from))
}

pub async fn update(
    db: &impl ConnectionTrait,
    id: Uuid,
    input: &SchoolInput,
) -> Result<Option<School>, DbErr> {
    let Some(model) = school::Entity::find_by_id(id).one(db).await? else {
        return Ok(None);
    };
    let mut model: school::ActiveModel = model.into();
    model.name = Set(input.name.clone());
    model.lon = Set(input.lon);
    model.lat = Set(input.lat);
    Ok(Some(model.update(db).await?.into()))
}

pub async fn delete(db: &impl ConnectionTrait, id: Uuid) -> Result<bool, DbErr> {
    let deleted = school::Entity::delete_by_id(id).exec(db).await?;
    Ok(deleted.rows_affected > 0)
}

pub async fn page(db: &impl ConnectionTrait, after: Uuid, size: u64) -> Result<Vec<School>, DbErr> {
    let models = school::Entity::find()
        .filter(school::Column::Id.gt(after))
        .order_by_asc(school::Column::Id)
        .limit(size)
        .all(db)
        .await?;
    Ok(models.into_iter().map(School::from).collect())
}
