use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m004_hopstand_temp_rename"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("sql/004_hopstand_temp_rename.sql"))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE recipes RENAME COLUMN hopstand_temp_c TO whirlpool_temp_c;
                 ALTER TABLE recipe_addition_hops RENAME COLUMN hopstand_temp_c TO whirlpool_temp_c;",
            )
            .await?;
        Ok(())
    }
}
