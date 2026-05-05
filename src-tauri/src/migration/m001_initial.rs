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
        let tables = [
            "settings", "mash_steps", "mashes",
            "recipe_addition_waters", "recipe_addition_miscs",
            "recipe_addition_yeasts", "recipe_addition_hops",
            "recipe_addition_fermentables", "recipes",
            "waters", "miscs", "yeasts", "hops", "fermentables",
            "equipment_profiles", "styles",
        ];
        for table in tables {
            manager
                .get_connection()
                .execute_unprepared(&format!("DROP TABLE IF EXISTS {table}"))
                .await?;
        }
        Ok(())
    }
}
