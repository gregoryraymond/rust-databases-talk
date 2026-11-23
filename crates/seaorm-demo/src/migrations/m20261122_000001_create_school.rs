use sea_orm_migration::{
    prelude::*,
    schema::{double, pk_uuid, string, timestamp_with_time_zone},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum School {
    Table,
    Id,
    Name,
    Lon,
    Lat,
    CreatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(School::Table)
                    .col(pk_uuid(School::Id))
                    .col(string(School::Name))
                    .col(double(School::Lon))
                    .col(double(School::Lat))
                    .col(timestamp_with_time_zone(School::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(School::Table).to_owned())
            .await
    }
}
