use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() != DbBackend::Postgres {
            return Ok(());
        }
        let db = manager.get_connection();
        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS postgis")
            .await?;
        db.execute_unprepared(
            "CREATE TABLE suburb (name TEXT PRIMARY KEY, outline geometry(Polygon, 4326) NOT NULL)",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .get_connection()
                .execute_unprepared("DROP TABLE suburb")
                .await?;
        }
        Ok(())
    }
}
