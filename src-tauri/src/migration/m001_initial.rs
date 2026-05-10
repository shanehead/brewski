use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str { "m001_initial" }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("sql/001_initial.sql"))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TABLE IF EXISTS mash_steps;
                 DROP TABLE IF EXISTS mashes;
                 DROP TABLE IF EXISTS recipe_addition_waters;
                 DROP TABLE IF EXISTS recipe_addition_miscs;
                 DROP TABLE IF EXISTS recipe_addition_yeasts;
                 DROP TABLE IF EXISTS recipe_addition_hops;
                 DROP TABLE IF EXISTS recipe_addition_fermentables;
                 DROP TABLE IF EXISTS recipes;
                 DROP TABLE IF EXISTS waters;
                 DROP TABLE IF EXISTS miscs;
                 DROP TABLE IF EXISTS yeasts;
                 DROP TABLE IF EXISTS hops;
                 DROP TABLE IF EXISTS fermentables;
                 DROP TABLE IF EXISTS equipment_profiles;
                 DROP TABLE IF EXISTS styles;
                 DROP TABLE IF EXISTS settings;",
            )
            .await?;
        Ok(())
    }
}
