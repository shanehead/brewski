use sea_orm_migration::prelude::*;

mod m001_initial;
mod m002_seed_data;
mod m003_strike_temp;
mod m004_yeast_extended_fields;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_initial::Migration),
            Box::new(m002_seed_data::Migration),
            Box::new(m003_strike_temp::Migration),
            Box::new(m004_yeast_extended_fields::Migration),
        ]
    }
}
