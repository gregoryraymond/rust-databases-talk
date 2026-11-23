mod m20261122_000001_create_school;
mod m20261122_000002_create_suburb;

use sea_orm_migration::{MigrationTrait, MigratorTrait, async_trait::async_trait};

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261122_000001_create_school::Migration),
            Box::new(m20261122_000002_create_suburb::Migration),
        ]
    }
}
