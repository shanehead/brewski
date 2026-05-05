use sea_orm_migration::prelude::*;

mod m001_initial;
mod m002_seed_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_initial::Migration),
            Box::new(m002_seed_data::Migration),
        ]
    }
}
