use sqlx::SqlitePool;
use crate::error::AppError;
use crate::models::{Recipe, RecipeRow, RecipeSummary, CreateRecipeInput, UpdateRecipeInput};
use uuid::Uuid;

pub async fn list(db: &SqlitePool) -> Result<Vec<RecipeSummary>, AppError> {
    let rows = sqlx::query_as::<_, RecipeSummary>(
        "SELECT r.id, r.name, r.type, r.batch_size_l, r.created_at, r.updated_at,
                s.name as style_name
         FROM recipes r
         LEFT JOIN styles s ON r.style_id = s.id
         ORDER BY r.updated_at DESC"
    )
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn get(db: &SqlitePool, id: &str) -> Result<Recipe, AppError> {
    let row = sqlx::query_as::<_, RecipeRow>(
        "SELECT * FROM recipes WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or(AppError::NotFound)?;

    let fermentables = sqlx::query_as::<_, crate::models::RecipeAdditionFermentable>(
        "SELECT * FROM recipe_addition_fermentables WHERE recipe_id = ? ORDER BY addition_order"
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let hops = sqlx::query_as::<_, crate::models::RecipeAdditionHop>(
        "SELECT * FROM recipe_addition_hops WHERE recipe_id = ? ORDER BY addition_order"
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let yeasts = sqlx::query_as::<_, crate::models::RecipeAdditionYeast>(
        "SELECT * FROM recipe_addition_yeasts WHERE recipe_id = ?"
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let miscs = sqlx::query_as::<_, crate::models::RecipeAdditionMisc>(
        "SELECT * FROM recipe_addition_miscs WHERE recipe_id = ? ORDER BY addition_order"
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let waters = sqlx::query_as::<_, crate::models::RecipeAdditionWater>(
        "SELECT * FROM recipe_addition_waters WHERE recipe_id = ?"
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let mash_row = sqlx::query_as::<_, crate::models::MashRow>(
        "SELECT * FROM mashes WHERE recipe_id = ?"
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    let mash = if let Some(mash_row) = mash_row {
        let steps = sqlx::query_as::<_, crate::models::MashStep>(
            "SELECT * FROM mash_steps WHERE mash_id = ? ORDER BY step_order"
        )
        .bind(&mash_row.id)
        .fetch_all(db)
        .await?;
        Some(crate::models::Mash {
            id: mash_row.id,
            recipe_id: mash_row.recipe_id,
            name: mash_row.name,
            grain_temp_c: mash_row.grain_temp_c,
            tun_temp_c: mash_row.tun_temp_c,
            sparge_temp_c: mash_row.sparge_temp_c,
            ph: mash_row.ph,
            tun_weight_kg: mash_row.tun_weight_kg,
            tun_specific_heat: mash_row.tun_specific_heat,
            equip_adjust: mash_row.equip_adjust,
            notes: mash_row.notes,
            steps,
        })
    } else {
        None
    };

    let equipment_profile = if let Some(ref ep_id) = row.equipment_profile_id {
        sqlx::query_as::<_, crate::models::EquipmentProfile>(
            "SELECT * FROM equipment_profiles WHERE id = ?"
        )
        .bind(ep_id)
        .fetch_optional(db)
        .await?
    } else {
        None
    };

    let style = if let Some(ref s_id) = row.style_id {
        sqlx::query_as::<_, crate::models::Style>(
            "SELECT * FROM styles WHERE id = ?"
        )
        .bind(s_id)
        .fetch_optional(db)
        .await?
    } else {
        None
    };

    Ok(Recipe {
        id: row.id,
        name: row.name,
        type_: row.type_,
        brewer: row.brewer,
        asst_brewer: row.asst_brewer,
        batch_size_l: row.batch_size_l,
        boil_size_l: row.boil_size_l,
        boil_time_min: row.boil_time_min,
        efficiency_pct: row.efficiency_pct,
        style_id: row.style_id,
        equipment_profile_id: row.equipment_profile_id,
        notes: row.notes,
        taste_notes: row.taste_notes,
        taste_rating: row.taste_rating,
        og: row.og,
        fg: row.fg,
        fermentation_stages: row.fermentation_stages,
        primary_age_days: row.primary_age_days,
        primary_temp_c: row.primary_temp_c,
        secondary_age_days: row.secondary_age_days,
        secondary_temp_c: row.secondary_temp_c,
        tertiary_age_days: row.tertiary_age_days,
        tertiary_temp_c: row.tertiary_temp_c,
        age_days: row.age_days,
        age_temp_c: row.age_temp_c,
        carbonation_vols: row.carbonation_vols,
        forced_carbonation: row.forced_carbonation,
        priming_sugar_name: row.priming_sugar_name,
        carbonation_temp_c: row.carbonation_temp_c,
        priming_sugar_equiv: row.priming_sugar_equiv,
        keg_priming_factor: row.keg_priming_factor,
        date: row.date,
        created_at: row.created_at,
        updated_at: row.updated_at,
        equipment_profile,
        style,
        fermentables,
        hops,
        yeasts,
        miscs,
        waters,
        mash,
    })
}

pub async fn create(db: &SqlitePool, input: CreateRecipeInput) -> Result<Recipe, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = now_secs();

    let (batch_size, boil_size, boil_time, equipment_profile_id) = if let Some(ref src_id) = input.source_id {
        let src = get(db, src_id).await?;
        (src.batch_size_l, src.boil_size_l, src.boil_time_min, src.equipment_profile_id)
    } else {
        (
            input.batch_size_l.unwrap_or(23.0),
            input.boil_size_l.unwrap_or(27.0),
            input.boil_time_min.unwrap_or(60.0),
            input.equipment_profile_id,
        )
    };

    sqlx::query(
        "INSERT INTO recipes (id, name, type, batch_size_l, boil_size_l, boil_time_min, equipment_profile_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&input.name)
    .bind(input.type_.as_deref().unwrap_or("all_grain"))
    .bind(batch_size)
    .bind(boil_size)
    .bind(boil_time)
    .bind(&equipment_profile_id)
    .bind(now)
    .bind(now)
    .execute(db)
    .await?;

    if let Some(src_id) = input.source_id {
        let src_fermentables = sqlx::query_as::<_, crate::models::RecipeAdditionFermentable>(
            "SELECT * FROM recipe_addition_fermentables WHERE recipe_id = ?"
        ).bind(&src_id).fetch_all(db).await?;
        for f in src_fermentables {
            sqlx::query("INSERT INTO recipe_addition_fermentables (id, recipe_id, fermentable_id, name, type, yield_pct, color_lovibond, amount_kg, add_after_boil, addition_order) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(Uuid::new_v4().to_string()).bind(&id).bind(f.fermentable_id).bind(f.name).bind(f.type_).bind(f.yield_pct).bind(f.color_lovibond).bind(f.amount_kg).bind(f.add_after_boil).bind(f.addition_order)
                .execute(db).await?;
        }

        let src_hops = sqlx::query_as::<_, crate::models::RecipeAdditionHop>(
            "SELECT * FROM recipe_addition_hops WHERE recipe_id = ?"
        ).bind(&src_id).fetch_all(db).await?;
        for h in src_hops {
            sqlx::query("INSERT INTO recipe_addition_hops (id, recipe_id, hop_id, name, alpha_pct, form, amount_kg, use, time_min, addition_order) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(Uuid::new_v4().to_string()).bind(&id).bind(h.hop_id).bind(h.name).bind(h.alpha_pct).bind(h.form).bind(h.amount_kg).bind(h.use_).bind(h.time_min).bind(h.addition_order)
                .execute(db).await?;
        }

        let src_yeasts = sqlx::query_as::<_, crate::models::RecipeAdditionYeast>(
            "SELECT * FROM recipe_addition_yeasts WHERE recipe_id = ?"
        ).bind(&src_id).fetch_all(db).await?;
        for y in src_yeasts {
            sqlx::query("INSERT INTO recipe_addition_yeasts (id, recipe_id, yeast_id, name, type, form, laboratory, product_id, attenuation_pct, amount, amount_is_weight, add_to_secondary, times_cultured) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(Uuid::new_v4().to_string()).bind(&id).bind(y.yeast_id).bind(y.name).bind(y.type_).bind(y.form).bind(y.laboratory).bind(y.product_id).bind(y.attenuation_pct).bind(y.amount).bind(y.amount_is_weight).bind(y.add_to_secondary).bind(y.times_cultured)
                .execute(db).await?;
        }
    }

    get(db, &id).await
}

pub async fn update(db: &SqlitePool, id: &str, input: UpdateRecipeInput) -> Result<Recipe, AppError> {
    let current = get(db, id).await?;
    let now = now_secs();

    sqlx::query(
        "UPDATE recipes SET
            name = ?, type = ?, brewer = ?, asst_brewer = ?,
            batch_size_l = ?, boil_size_l = ?, boil_time_min = ?,
            efficiency_pct = ?, style_id = ?, equipment_profile_id = ?,
            notes = ?, taste_notes = ?, taste_rating = ?,
            fermentation_stages = ?, primary_age_days = ?, primary_temp_c = ?,
            secondary_age_days = ?, secondary_temp_c = ?, tertiary_age_days = ?,
            tertiary_temp_c = ?, age_days = ?, age_temp_c = ?,
            carbonation_vols = ?, forced_carbonation = ?,
            priming_sugar_name = ?, carbonation_temp_c = ?, date = ?,
            updated_at = ?
         WHERE id = ?"
    )
    .bind(input.name.unwrap_or(current.name))
    .bind(input.type_.unwrap_or(current.type_))
    .bind(input.brewer.or(current.brewer))
    .bind(input.asst_brewer.or(current.asst_brewer))
    .bind(input.batch_size_l.unwrap_or(current.batch_size_l))
    .bind(input.boil_size_l.unwrap_or(current.boil_size_l))
    .bind(input.boil_time_min.unwrap_or(current.boil_time_min))
    .bind(input.efficiency_pct.or(current.efficiency_pct))
    .bind(input.style_id.or(current.style_id))
    .bind(input.equipment_profile_id.or(current.equipment_profile_id))
    .bind(input.notes.or(current.notes))
    .bind(input.taste_notes.or(current.taste_notes))
    .bind(input.taste_rating.or(current.taste_rating))
    .bind(input.fermentation_stages.unwrap_or(current.fermentation_stages))
    .bind(input.primary_age_days.or(current.primary_age_days))
    .bind(input.primary_temp_c.or(current.primary_temp_c))
    .bind(input.secondary_age_days.or(current.secondary_age_days))
    .bind(input.secondary_temp_c.or(current.secondary_temp_c))
    .bind(input.tertiary_age_days.or(current.tertiary_age_days))
    .bind(input.tertiary_temp_c.or(current.tertiary_temp_c))
    .bind(input.age_days.or(current.age_days))
    .bind(input.age_temp_c.or(current.age_temp_c))
    .bind(input.carbonation_vols.or(current.carbonation_vols))
    .bind(input.forced_carbonation.unwrap_or(current.forced_carbonation))
    .bind(input.priming_sugar_name.or(current.priming_sugar_name))
    .bind(input.carbonation_temp_c.or(current.carbonation_temp_c))
    .bind(input.date.or(current.date))
    .bind(now)
    .bind(id)
    .execute(db)
    .await?;

    get(db, id).await
}

pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM recipes WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::setup_test_db;

    fn basic_create_input() -> CreateRecipeInput {
        CreateRecipeInput {
            name: "Test Recipe".into(),
            type_: Some("all_grain".into()),
            batch_size_l: Some(23.0),
            boil_size_l: Some(27.0),
            boil_time_min: Some(60.0),
            equipment_profile_id: None,
            source_id: None,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        create(&db, basic_create_input()).await.unwrap();
        let all = list(&db).await.unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].name, "Test Recipe");
    }

    #[tokio::test]
    async fn test_get_returns_full_recipe() {
        let db = setup_test_db().await;
        let created = create(&db, basic_create_input()).await.unwrap();
        let fetched = get(&db, &created.id).await.unwrap();
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.batch_size_l, 23.0);
        assert!(fetched.fermentables.is_empty());
    }

    #[tokio::test]
    async fn test_update_name() {
        let db = setup_test_db().await;
        let created = create(&db, basic_create_input()).await.unwrap();
        let updated = update(&db, &created.id, UpdateRecipeInput {
            name: Some("Updated Name".into()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(updated.name, "Updated Name");
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let created = create(&db, basic_create_input()).await.unwrap();
        delete(&db, &created.id).await.unwrap();
        let all = list(&db).await.unwrap();
        assert!(all.is_empty());
    }

    #[tokio::test]
    async fn test_duplicate_via_source_id() {
        let db = setup_test_db().await;
        let original = create(&db, basic_create_input()).await.unwrap();
        let dupe = create(&db, CreateRecipeInput {
            name: "Copy of Test Recipe".into(),
            source_id: Some(original.id.clone()),
            ..Default::default()
        }).await.unwrap();
        assert_ne!(dupe.id, original.id);
        assert_eq!(dupe.batch_size_l, original.batch_size_l);
    }
}
