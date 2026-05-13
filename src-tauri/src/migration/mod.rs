use sea_orm_migration::prelude::*;

mod m001_initial;
mod m002_water_chemistry;
mod m003_whirlpool_temp;
mod m004_hopstand_temp_rename;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_initial::Migration),
            Box::new(m002_water_chemistry::Migration),
            Box::new(m003_whirlpool_temp::Migration),
            Box::new(m004_hopstand_temp_rename::Migration),
        ]
    }
}
