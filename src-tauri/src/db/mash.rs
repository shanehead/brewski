use sqlx::SqlitePool;
use crate::error::AppError;
use crate::models::{Mash, MashRow, MashStep, UpdateMashInput, CreateMashStepInput, UpdateMashStepInput};
use uuid::Uuid;

fn new_id() -> String { Uuid::new_v4().to_string() }

async fn fetch_mash(db: &SqlitePool, mash_id: &str) -> Result<Mash, AppError> {
    let row = sqlx::query_as::<_, MashRow>("SELECT * FROM mashes WHERE id = ?")
        .bind(mash_id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;
    let steps = sqlx::query_as::<_, MashStep>(
        "SELECT * FROM mash_steps WHERE mash_id = ? ORDER BY step_order"
    ).bind(mash_id).fetch_all(db).await?;
    Ok(Mash {
        id: row.id, recipe_id: row.recipe_id, name: row.name,
        grain_temp_c: row.grain_temp_c, tun_temp_c: row.tun_temp_c,
        sparge_temp_c: row.sparge_temp_c, ph: row.ph,
        tun_weight_kg: row.tun_weight_kg, tun_specific_heat: row.tun_specific_heat,
        equip_adjust: row.equip_adjust, notes: row.notes, steps,
    })
}

pub async fn get_for_recipe(db: &SqlitePool, recipe_id: &str) -> Result<Mash, AppError> {
    let row = sqlx::query_as::<_, MashRow>("SELECT * FROM mashes WHERE recipe_id = ?")
        .bind(recipe_id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;
    fetch_mash(db, &row.id).await
}

pub async fn upsert_for_recipe(db: &SqlitePool, recipe_id: &str, input: UpdateMashInput) -> Result<Mash, AppError> {
    let existing = sqlx::query_as::<_, MashRow>("SELECT * FROM mashes WHERE recipe_id = ?")
        .bind(recipe_id).fetch_optional(db).await?;

    let mash_id = if let Some(row) = existing {
        sqlx::query(
            "UPDATE mashes SET name = ?, grain_temp_c = ?, tun_temp_c = ?, sparge_temp_c = ?, ph = ?, notes = ? WHERE id = ?"
        )
        .bind(input.name.unwrap_or(row.name))
        .bind(input.grain_temp_c.unwrap_or(row.grain_temp_c))
        .bind(input.tun_temp_c.or(row.tun_temp_c))
        .bind(input.sparge_temp_c.or(row.sparge_temp_c))
        .bind(input.ph.or(row.ph))
        .bind(input.notes.or(row.notes))
        .bind(&row.id)
        .execute(db).await?;
        row.id
    } else {
        let id = new_id();
        sqlx::query(
            "INSERT INTO mashes (id, recipe_id, name, grain_temp_c, tun_temp_c, sparge_temp_c, ph, notes) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id).bind(recipe_id)
        .bind(input.name.as_deref().unwrap_or("Single Infusion"))
        .bind(input.grain_temp_c.unwrap_or(21.0))
        .bind(input.tun_temp_c).bind(input.sparge_temp_c).bind(input.ph).bind(input.notes)
        .execute(db).await?;
        id
    };

    fetch_mash(db, &mash_id).await
}

pub async fn create_step(db: &SqlitePool, mash_id: &str, input: CreateMashStepInput) -> Result<MashStep, AppError> {
    let id = new_id();
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM mash_steps WHERE mash_id = ?")
        .bind(mash_id).fetch_one(db).await?;

    sqlx::query(
        "INSERT INTO mash_steps (id, mash_id, name, type, infuse_amount_l, step_temp_c, step_time_min, ramp_time_min, end_temp_c, step_order)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(mash_id).bind(&input.name)
    .bind(input.type_.as_deref().unwrap_or("infusion"))
    .bind(input.infuse_amount_l).bind(input.step_temp_c).bind(input.step_time_min)
    .bind(input.ramp_time_min).bind(input.end_temp_c).bind(count.0)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, MashStep>("SELECT * FROM mash_steps WHERE id = ?")
        .bind(&id).fetch_one(db).await?)
}

pub async fn update_step(db: &SqlitePool, id: &str, input: UpdateMashStepInput) -> Result<MashStep, AppError> {
    let current = sqlx::query_as::<_, MashStep>("SELECT * FROM mash_steps WHERE id = ?")
        .bind(id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;

    sqlx::query(
        "UPDATE mash_steps SET name = ?, type = ?, infuse_amount_l = ?, step_temp_c = ?, step_time_min = ?, ramp_time_min = ?, end_temp_c = ? WHERE id = ?"
    )
    .bind(input.name.as_deref().unwrap_or(&current.name))
    .bind(input.type_.as_deref().unwrap_or(&current.type_))
    .bind(input.infuse_amount_l.or(current.infuse_amount_l))
    .bind(input.step_temp_c.unwrap_or(current.step_temp_c))
    .bind(input.step_time_min.unwrap_or(current.step_time_min))
    .bind(input.ramp_time_min.or(current.ramp_time_min))
    .bind(input.end_temp_c.or(current.end_temp_c))
    .bind(id)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, MashStep>("SELECT * FROM mash_steps WHERE id = ?")
        .bind(id).fetch_one(db).await?)
}

pub async fn delete_step(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM mash_steps WHERE id = ?").bind(id).execute(db).await?;
    Ok(())
}

pub async fn update_step_order(db: &SqlitePool, ordered_ids: Vec<String>) -> Result<(), AppError> {
    for (i, id) in ordered_ids.iter().enumerate() {
        sqlx::query("UPDATE mash_steps SET step_order = ? WHERE id = ?")
            .bind(i as i64).bind(id).execute(db).await?;
    }
    Ok(())
}
