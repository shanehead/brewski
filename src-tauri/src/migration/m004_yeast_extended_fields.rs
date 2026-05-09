use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str { "m004_yeast_extended_fields" }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("sql/004_yeast_extended_fields.sql"))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();
        for col in &[
            "min_attenuation_pct", "max_attenuation_pct", "alcohol_tolerance",
            "flavor_profile", "styles", "substitutes", "species",
            "pof_positive", "sta1_positive",
        ] {
            conn.execute_unprepared(&format!("ALTER TABLE yeasts DROP COLUMN {col}"))
                .await?;
        }
        Ok(())
    }
}
