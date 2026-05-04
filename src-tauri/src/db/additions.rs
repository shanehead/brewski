use sqlx::SqlitePool;
use crate::error::AppError;
use crate::models::{
    RecipeAdditionFermentable, RecipeAdditionHop, RecipeAdditionYeast,
    RecipeAdditionMisc, RecipeAdditionWater,
    CreateFermentableAdditionInput, UpdateFermentableAdditionInput,
    CreateHopAdditionInput, UpdateHopAdditionInput,
    CreateYeastAdditionInput, UpdateYeastAdditionInput,
    CreateMiscAdditionInput, UpdateMiscAdditionInput,
    CreateWaterAdditionInput, UpdateWaterAdditionInput,
};
use uuid::Uuid;

fn new_id() -> String { Uuid::new_v4().to_string() }

pub async fn create_fermentable(
    db: &SqlitePool,
    recipe_id: &str,
    input: CreateFermentableAdditionInput,
) -> Result<RecipeAdditionFermentable, AppError> {
    let id = new_id();
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM recipe_addition_fermentables WHERE recipe_id = ?"
    ).bind(recipe_id).fetch_one(db).await?;

    sqlx::query(
        "INSERT INTO recipe_addition_fermentables
            (id, recipe_id, fermentable_id, name, type, yield_pct, color_lovibond, amount_kg, add_after_boil, addition_order)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(recipe_id).bind(&input.fermentable_id).bind(&input.name)
    .bind(&input.type_).bind(input.yield_pct).bind(input.color_lovibond)
    .bind(input.amount_kg).bind(input.add_after_boil.unwrap_or(false))
    .bind(count.0)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionFermentable>(
        "SELECT * FROM recipe_addition_fermentables WHERE id = ?"
    ).bind(&id).fetch_one(db).await?)
}

pub async fn update_fermentable(
    db: &SqlitePool,
    id: &str,
    input: UpdateFermentableAdditionInput,
) -> Result<RecipeAdditionFermentable, AppError> {
    let current = sqlx::query_as::<_, RecipeAdditionFermentable>(
        "SELECT * FROM recipe_addition_fermentables WHERE id = ?"
    ).bind(id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;

    sqlx::query(
        "UPDATE recipe_addition_fermentables SET amount_kg = ?, add_after_boil = ?, addition_order = ? WHERE id = ?"
    )
    .bind(input.amount_kg.unwrap_or(current.amount_kg))
    .bind(input.add_after_boil.unwrap_or(current.add_after_boil))
    .bind(input.addition_order.unwrap_or(current.addition_order))
    .bind(id)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionFermentable>(
        "SELECT * FROM recipe_addition_fermentables WHERE id = ?"
    ).bind(id).fetch_one(db).await?)
}

pub async fn delete_fermentable(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM recipe_addition_fermentables WHERE id = ?")
        .bind(id).execute(db).await?;
    Ok(())
}

pub async fn create_hop(
    db: &SqlitePool,
    recipe_id: &str,
    input: CreateHopAdditionInput,
) -> Result<RecipeAdditionHop, AppError> {
    let id = new_id();
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM recipe_addition_hops WHERE recipe_id = ?"
    ).bind(recipe_id).fetch_one(db).await?;

    sqlx::query(
        "INSERT INTO recipe_addition_hops
            (id, recipe_id, hop_id, name, alpha_pct, form, amount_kg, use, time_min, addition_order)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(recipe_id).bind(&input.hop_id).bind(&input.name)
    .bind(input.alpha_pct).bind(input.form.as_deref().unwrap_or("pellet"))
    .bind(input.amount_kg).bind(&input.use_).bind(input.time_min).bind(count.0)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionHop>(
        "SELECT * FROM recipe_addition_hops WHERE id = ?"
    ).bind(&id).fetch_one(db).await?)
}

pub async fn update_hop(
    db: &SqlitePool,
    id: &str,
    input: UpdateHopAdditionInput,
) -> Result<RecipeAdditionHop, AppError> {
    let current = sqlx::query_as::<_, RecipeAdditionHop>(
        "SELECT * FROM recipe_addition_hops WHERE id = ?"
    ).bind(id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;

    sqlx::query(
        "UPDATE recipe_addition_hops SET amount_kg = ?, use = ?, time_min = ?, addition_order = ? WHERE id = ?"
    )
    .bind(input.amount_kg.unwrap_or(current.amount_kg))
    .bind(input.use_.as_deref().unwrap_or(&current.use_))
    .bind(input.time_min.unwrap_or(current.time_min))
    .bind(input.addition_order.unwrap_or(current.addition_order))
    .bind(id)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionHop>(
        "SELECT * FROM recipe_addition_hops WHERE id = ?"
    ).bind(id).fetch_one(db).await?)
}

pub async fn delete_hop(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM recipe_addition_hops WHERE id = ?")
        .bind(id).execute(db).await?;
    Ok(())
}

pub async fn create_yeast(
    db: &SqlitePool,
    recipe_id: &str,
    input: CreateYeastAdditionInput,
) -> Result<RecipeAdditionYeast, AppError> {
    let id = new_id();
    sqlx::query(
        "INSERT INTO recipe_addition_yeasts
            (id, recipe_id, yeast_id, name, type, form, laboratory, product_id, attenuation_pct, amount, amount_is_weight)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(recipe_id).bind(&input.yeast_id).bind(&input.name)
    .bind(&input.type_).bind(&input.form).bind(&input.laboratory).bind(&input.product_id)
    .bind(input.attenuation_pct).bind(input.amount)
    .bind(input.amount_is_weight.unwrap_or(false))
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionYeast>(
        "SELECT * FROM recipe_addition_yeasts WHERE id = ?"
    ).bind(&id).fetch_one(db).await?)
}

pub async fn update_yeast(
    db: &SqlitePool,
    id: &str,
    input: UpdateYeastAdditionInput,
) -> Result<RecipeAdditionYeast, AppError> {
    let current = sqlx::query_as::<_, RecipeAdditionYeast>(
        "SELECT * FROM recipe_addition_yeasts WHERE id = ?"
    ).bind(id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;

    sqlx::query(
        "UPDATE recipe_addition_yeasts SET attenuation_pct = ?, amount = ?, amount_is_weight = ?, add_to_secondary = ?, times_cultured = ? WHERE id = ?"
    )
    .bind(input.attenuation_pct.or(current.attenuation_pct))
    .bind(input.amount.or(current.amount))
    .bind(input.amount_is_weight.unwrap_or(current.amount_is_weight))
    .bind(input.add_to_secondary.unwrap_or(current.add_to_secondary))
    .bind(input.times_cultured.unwrap_or(current.times_cultured))
    .bind(id)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionYeast>(
        "SELECT * FROM recipe_addition_yeasts WHERE id = ?"
    ).bind(id).fetch_one(db).await?)
}

pub async fn delete_yeast(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM recipe_addition_yeasts WHERE id = ?")
        .bind(id).execute(db).await?;
    Ok(())
}

pub async fn create_misc(
    db: &SqlitePool,
    recipe_id: &str,
    input: CreateMiscAdditionInput,
) -> Result<RecipeAdditionMisc, AppError> {
    let id = new_id();
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM recipe_addition_miscs WHERE recipe_id = ?"
    ).bind(recipe_id).fetch_one(db).await?;

    sqlx::query(
        "INSERT INTO recipe_addition_miscs
            (id, recipe_id, misc_id, name, type, use, amount, amount_is_weight, time_min, addition_order)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(recipe_id).bind(&input.misc_id).bind(&input.name)
    .bind(&input.type_).bind(&input.use_).bind(input.amount)
    .bind(input.amount_is_weight.unwrap_or(false)).bind(input.time_min).bind(count.0)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionMisc>(
        "SELECT * FROM recipe_addition_miscs WHERE id = ?"
    ).bind(&id).fetch_one(db).await?)
}

pub async fn update_misc(
    db: &SqlitePool,
    id: &str,
    input: UpdateMiscAdditionInput,
) -> Result<RecipeAdditionMisc, AppError> {
    let current = sqlx::query_as::<_, RecipeAdditionMisc>(
        "SELECT * FROM recipe_addition_miscs WHERE id = ?"
    ).bind(id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;

    sqlx::query(
        "UPDATE recipe_addition_miscs SET amount = ?, amount_is_weight = ?, use = ?, time_min = ?, addition_order = ? WHERE id = ?"
    )
    .bind(input.amount.unwrap_or(current.amount))
    .bind(input.amount_is_weight.unwrap_or(current.amount_is_weight))
    .bind(input.use_.as_deref().unwrap_or(&current.use_))
    .bind(input.time_min.unwrap_or(current.time_min))
    .bind(input.addition_order.unwrap_or(current.addition_order))
    .bind(id)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionMisc>(
        "SELECT * FROM recipe_addition_miscs WHERE id = ?"
    ).bind(id).fetch_one(db).await?)
}

pub async fn delete_misc(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM recipe_addition_miscs WHERE id = ?")
        .bind(id).execute(db).await?;
    Ok(())
}

pub async fn create_water(
    db: &SqlitePool,
    recipe_id: &str,
    input: CreateWaterAdditionInput,
) -> Result<RecipeAdditionWater, AppError> {
    let id = new_id();
    sqlx::query(
        "INSERT INTO recipe_addition_waters (id, recipe_id, water_id, name, amount_l) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(recipe_id).bind(&input.water_id).bind(&input.name).bind(input.amount_l)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionWater>(
        "SELECT * FROM recipe_addition_waters WHERE id = ?"
    ).bind(&id).fetch_one(db).await?)
}

pub async fn update_water(
    db: &SqlitePool,
    id: &str,
    input: UpdateWaterAdditionInput,
) -> Result<RecipeAdditionWater, AppError> {
    let current = sqlx::query_as::<_, RecipeAdditionWater>(
        "SELECT * FROM recipe_addition_waters WHERE id = ?"
    ).bind(id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;

    sqlx::query("UPDATE recipe_addition_waters SET amount_l = ? WHERE id = ?")
        .bind(input.amount_l.unwrap_or(current.amount_l))
        .bind(id)
        .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionWater>(
        "SELECT * FROM recipe_addition_waters WHERE id = ?"
    ).bind(id).fetch_one(db).await?)
}

pub async fn delete_water(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM recipe_addition_waters WHERE id = ?")
        .bind(id).execute(db).await?;
    Ok(())
}
