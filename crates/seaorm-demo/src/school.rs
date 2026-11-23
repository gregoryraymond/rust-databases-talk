use bench_common::school::School;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "school")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub lon: f64,
    pub lat: f64,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for School {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            lon: m.lon,
            lat: m.lat,
            created_at: m.created_at,
        }
    }
}
