use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m002_water_chemistry"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("sql/002_water_chemistry.sql"))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TABLE IF EXISTS recipe_water_adjustments;
                 ALTER TABLE recipes DROP COLUMN IF EXISTS mash_water_id;
                 ALTER TABLE recipes DROP COLUMN IF EXISTS sparge_water_id;",
            )
            .await?;
        Ok(())
    }
}
