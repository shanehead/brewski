use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m003_whirlpool_temp"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("sql/003_whirlpool_temp.sql"))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE recipes DROP COLUMN IF EXISTS whirlpool_temp_c;
                 ALTER TABLE recipe_addition_hops DROP COLUMN IF EXISTS whirlpool_temp_c;",
            )
            .await?;
        Ok(())
    }
}
