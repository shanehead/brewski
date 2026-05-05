use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set};

use crate::entities::equipment_profiles;
use crate::error::AppError;
use crate::models::{CreateEquipmentProfileInput, EquipmentProfile, UpdateEquipmentProfileInput};

use super::{new_id, now_secs, to_dec, to_dec_opt};

pub struct EquipmentRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> EquipmentRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self) -> Result<Vec<EquipmentProfile>, AppError> {
        equipment_profiles::Entity::find()
            .order_by_asc(equipment_profiles::Column::Name)
            .all(self.db)
            .await?
            .into_iter()
            .map(EquipmentProfile::try_from)
            .collect()
    }

    pub async fn get(&self, id: &str) -> Result<EquipmentProfile, AppError> {
        equipment_profiles::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(EquipmentProfile::try_from)
    }

    pub async fn create(
        &self,
        input: CreateEquipmentProfileInput,
    ) -> Result<EquipmentProfile, AppError> {
        let id = new_id();
        let now = now_secs() as i32;
        equipment_profiles::ActiveModel {
            id: Set(id.clone()),
            name: Set(input.name),
            notes: Set(input.notes),
            boil_size_l: Set(to_dec(input.boil_size_l)),
            batch_size_l: Set(to_dec(input.batch_size_l)),
            boil_time_min: Set(to_dec(input.boil_time_min.unwrap_or(60.0))),
            evap_rate_pct_hr: Set(to_dec_opt(input.evap_rate_pct_hr)),
            trub_chiller_loss_l: Set(to_dec_opt(input.trub_chiller_loss_l)),
            fermenter_loss_l: Set(to_dec_opt(input.fermenter_loss_l)),
            efficiency_pct: Set(to_dec(input.efficiency_pct)),
            calc_boil_volume: Set(0),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(self.db)
        .await?;
        self.get(&id).await
    }

    pub async fn update(
        &self,
        id: &str,
        input: UpdateEquipmentProfileInput,
    ) -> Result<EquipmentProfile, AppError> {
        let current = equipment_profiles::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;
        let mut active: equipment_profiles::ActiveModel = current.into();
        if let Some(v) = input.name {
            active.name = Set(v);
        }
        if let Some(v) = input.notes {
            active.notes = Set(Some(v));
        }
        if let Some(v) = input.boil_size_l {
            active.boil_size_l = Set(to_dec(v));
        }
        if let Some(v) = input.batch_size_l {
            active.batch_size_l = Set(to_dec(v));
        }
        if let Some(v) = input.boil_time_min {
            active.boil_time_min = Set(to_dec(v));
        }
        if let Some(v) = input.evap_rate_pct_hr {
            active.evap_rate_pct_hr = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.trub_chiller_loss_l {
            active.trub_chiller_loss_l = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.fermenter_loss_l {
            active.fermenter_loss_l = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.efficiency_pct {
            active.efficiency_pct = Set(to_dec(v));
        }
        active.updated_at = Set(now_secs() as i32);
        active.update(self.db).await?;
        self.get(id).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        equipment_profiles::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }
}

// Tests are written against the spec but require Task 13 to update
// `setup_test_db` to return `DatabaseConnection` before they will compile.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::setup_test_db;

    fn input() -> CreateEquipmentProfileInput {
        CreateEquipmentProfileInput {
            name: "10 Gallon Kettle".into(),
            notes: None,
            boil_size_l: 30.0,
            batch_size_l: 23.0,
            boil_time_min: Some(60.0),
            evap_rate_pct_hr: Some(10.0),
            trub_chiller_loss_l: Some(1.5),
            fermenter_loss_l: Some(1.0),
            efficiency_pct: 72.0,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let repo = EquipmentRepository::new(&db);
        let created = repo.create(input()).await.unwrap();
        assert_eq!(created.name, "10 Gallon Kettle");
        let all = repo.list().await.unwrap();
        assert!(all.iter().any(|e| e.id == created.id));
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let repo = EquipmentRepository::new(&db);
        let created = repo.create(input()).await.unwrap();
        let updated = repo
            .update(
                &created.id,
                UpdateEquipmentProfileInput {
                    name: Some("New Name".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(updated.name, "New Name");
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let repo = EquipmentRepository::new(&db);
        let created = repo.create(input()).await.unwrap();
        repo.delete(&created.id).await.unwrap();
        let all = repo.list().await.unwrap();
        assert!(!all.iter().any(|e| e.id == created.id));
    }
}
