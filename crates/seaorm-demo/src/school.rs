//! The school table as a SeaORM entity, without its geometry column.

use sea_orm::entity::prelude::*;

/// One school row's plain columns.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "school")]
pub struct Model {
    /// A time-ordered id.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// The school's name.
    pub name: String,
    /// Government, Catholic or Independent.
    pub sector: String,
    /// Primary, Secondary and so on.
    pub school_type: String,
    /// When the row was made.
    pub created_at: DateTimeUtc,
    /// When the row last changed.
    pub updated_at: DateTimeUtc,
}

/// The table has no relations SeaORM can express: suburbs join on `ST_Contains`.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
