mod m20240330_000001_create_patidar_users_table;

use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20240330_000001_create_patidar_users_table::Migration,
        )]
    }
}
