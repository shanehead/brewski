use sqlx::SqlitePool;
use crate::error::AppError;
use crate::models::{EquipmentProfile, CreateEquipmentProfileInput, UpdateEquipmentProfileInput};
use uuid::Uuid;

pub async fn list(db: &SqlitePool) -> Result<Vec<EquipmentProfile>, AppError> {
    let rows = sqlx::query_as::<_, EquipmentProfile>(
        "SELECT * FROM equipment_profiles ORDER BY name"
    )
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn get(db: &SqlitePool, id: &str) -> Result<EquipmentProfile, AppError> {
    sqlx::query_as::<_, EquipmentProfile>(
        "SELECT * FROM equipment_profiles WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or(AppError::NotFound)
}

pub async fn create(db: &SqlitePool, input: CreateEquipmentProfileInput) -> Result<EquipmentProfile, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    sqlx::query(
        "INSERT INTO equipment_profiles (
            id, name, notes, boil_size_l, batch_size_l, boil_time_min,
            evap_rate_pct_hr, trub_chiller_loss_l, fermenter_loss_l,
            efficiency_pct, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&input.name)
    .bind(&input.notes)
    .bind(input.boil_size_l)
    .bind(input.batch_size_l)
    .bind(input.boil_time_min.unwrap_or(60.0))
    .bind(input.evap_rate_pct_hr.unwrap_or(10.0))
    .bind(input.trub_chiller_loss_l.unwrap_or(1.0))
    .bind(input.fermenter_loss_l.unwrap_or(1.0))
    .bind(input.efficiency_pct)
    .bind(now)
    .bind(now)
    .execute(db)
    .await?;

    get(db, &id).await
}

pub async fn update(db: &SqlitePool, id: &str, input: UpdateEquipmentProfileInput) -> Result<EquipmentProfile, AppError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let current = get(db, id).await?;
    sqlx::query(
        "UPDATE equipment_profiles SET
            name = ?, notes = ?, boil_size_l = ?, batch_size_l = ?,
            boil_time_min = ?, evap_rate_pct_hr = ?, trub_chiller_loss_l = ?,
            fermenter_loss_l = ?, efficiency_pct = ?, updated_at = ?
        WHERE id = ?"
    )
    .bind(input.name.unwrap_or(current.name))
    .bind(input.notes.or(current.notes))
    .bind(input.boil_size_l.unwrap_or(current.boil_size_l))
    .bind(input.batch_size_l.unwrap_or(current.batch_size_l))
    .bind(input.boil_time_min.unwrap_or(current.boil_time_min))
    .bind(input.evap_rate_pct_hr.unwrap_or(current.evap_rate_pct_hr))
    .bind(input.trub_chiller_loss_l.unwrap_or(current.trub_chiller_loss_l))
    .bind(input.fermenter_loss_l.unwrap_or(current.fermenter_loss_l))
    .bind(input.efficiency_pct.unwrap_or(current.efficiency_pct))
    .bind(now)
    .bind(id)
    .execute(db)
    .await?;

    get(db, id).await
}

pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM equipment_profiles WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::setup_test_db;

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let input = CreateEquipmentProfileInput {
            name: "10 Gallon Kettle".into(),
            notes: None,
            boil_size_l: 30.0,
            batch_size_l: 23.0,
            boil_time_min: Some(60.0),
            evap_rate_pct_hr: Some(10.0),
            trub_chiller_loss_l: Some(1.5),
            fermenter_loss_l: Some(1.0),
            efficiency_pct: 72.0,
        };
        let created = create(&db, input).await.unwrap();
        assert_eq!(created.name, "10 Gallon Kettle");

        let all = list(&db).await.unwrap();
        assert!(all.len() >= 1);
        assert!(all.iter().any(|e| e.id == created.id));
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let created = create(&db, CreateEquipmentProfileInput {
            name: "Old Name".into(),
            notes: None,
            boil_size_l: 30.0,
            batch_size_l: 23.0,
            boil_time_min: None,
            evap_rate_pct_hr: None,
            trub_chiller_loss_l: None,
            fermenter_loss_l: None,
            efficiency_pct: 72.0,
        }).await.unwrap();

        let updated = update(&db, &created.id, UpdateEquipmentProfileInput {
            name: Some("New Name".into()),
            notes: None,
            boil_size_l: None,
            batch_size_l: None,
            boil_time_min: None,
            evap_rate_pct_hr: None,
            trub_chiller_loss_l: None,
            fermenter_loss_l: None,
            efficiency_pct: None,
        }).await.unwrap();
        assert_eq!(updated.name, "New Name");
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let created = create(&db, CreateEquipmentProfileInput {
            name: "To Delete".into(),
            notes: None,
            boil_size_l: 30.0,
            batch_size_l: 23.0,
            boil_time_min: None,
            evap_rate_pct_hr: None,
            trub_chiller_loss_l: None,
            fermenter_loss_l: None,
            efficiency_pct: 72.0,
        }).await.unwrap();

        delete(&db, &created.id).await.unwrap();
        let all = list(&db).await.unwrap();
        assert!(!all.iter().any(|e| e.id == created.id));
    }
}
