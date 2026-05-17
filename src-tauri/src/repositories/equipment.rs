use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set};

use crate::entities::equipment_profiles;
use crate::error::AppError;
use crate::models::{CreateEquipmentProfileInput, EquipmentProfile, UpdateEquipmentProfileInput};

use super::{new_id, now_secs};

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
            boil_size_l: Set(input.boil_size_l),
            batch_size_l: Set(input.batch_size_l),
            boil_time_min: Set(input.boil_time_min.unwrap_or(60.0)),
            evap_rate_pct_hr: Set(input.evap_rate_pct_hr),
            trub_chiller_loss_l: Set(input.trub_chiller_loss_l),
            fermenter_loss_l: Set(input.fermenter_loss_l),
            efficiency_pct: Set(input.efficiency_pct),
            calc_boil_volume: Set(1),
            batch_volume_target: Set(input
                .batch_volume_target
                .unwrap_or_else(|| "fermenter".into())),
            mash_tun_loss_l: Set(input.mash_tun_loss_l.unwrap_or(0.0)),
            hlt_deadspace_l: Set(input.hlt_deadspace_l),
            cooling_shrinkage_pct: Set(input.cooling_shrinkage_pct.unwrap_or(4.0)),
            calc_mash_efficiency: Set(input.calc_mash_efficiency.map(|b| b as i32).unwrap_or(1)),
            mash_efficiency_pct: Set(input.mash_efficiency_pct),
            calc_aroma_hop_utilization: Set(input
                .calc_aroma_hop_utilization
                .map(|b| b as i32)
                .unwrap_or(1)),
            aroma_hop_utilization_pct: Set(input.aroma_hop_utilization_pct.unwrap_or(23.0)),
            whirlpool_time_min: Set(input.whirlpool_time_min),
            altitude_adjustment: Set(input.altitude_adjustment.map(|b| b as i32).unwrap_or(0)),
            boil_temp_f: Set(input.boil_temp_f),
            sparge_method: Set(input.sparge_method.unwrap_or_else(|| "no_sparge".into())),
            mash_volume_min_l: Set(input.mash_volume_min_l),
            mash_volume_max_l: Set(input.mash_volume_max_l),
            sparge_volume_min_l: Set(input.sparge_volume_min_l),
            sparge_volume_max_l: Set(input.sparge_volume_max_l),
            calc_strike_water_temp: Set(input
                .calc_strike_water_temp
                .map(|b| b as i32)
                .unwrap_or(0)),
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
            active.boil_size_l = Set(v);
        }
        if let Some(v) = input.batch_size_l {
            active.batch_size_l = Set(v);
        }
        if let Some(v) = input.boil_time_min {
            active.boil_time_min = Set(v);
        }
        if let Some(v) = input.evap_rate_pct_hr {
            active.evap_rate_pct_hr = Set(Some(v));
        }
        if let Some(v) = input.trub_chiller_loss_l {
            active.trub_chiller_loss_l = Set(Some(v));
        }
        if let Some(v) = input.fermenter_loss_l {
            active.fermenter_loss_l = Set(Some(v));
        }
        if let Some(v) = input.efficiency_pct {
            active.efficiency_pct = Set(v);
        }
        if let Some(v) = input.batch_volume_target {
            active.batch_volume_target = Set(v);
        }
        if let Some(v) = input.mash_tun_loss_l {
            active.mash_tun_loss_l = Set(v);
        }
        active.hlt_deadspace_l = Set(input.hlt_deadspace_l);
        if let Some(v) = input.cooling_shrinkage_pct {
            active.cooling_shrinkage_pct = Set(v);
        }
        if let Some(v) = input.calc_mash_efficiency {
            active.calc_mash_efficiency = Set(v as i32);
        }
        active.mash_efficiency_pct = Set(input.mash_efficiency_pct);
        if let Some(v) = input.calc_aroma_hop_utilization {
            active.calc_aroma_hop_utilization = Set(v as i32);
        }
        if let Some(v) = input.aroma_hop_utilization_pct {
            active.aroma_hop_utilization_pct = Set(v);
        }
        active.whirlpool_time_min = Set(input.whirlpool_time_min);
        if let Some(v) = input.altitude_adjustment {
            active.altitude_adjustment = Set(v as i32);
        }
        active.boil_temp_f = Set(input.boil_temp_f);
        if let Some(v) = input.sparge_method {
            active.sparge_method = Set(v);
        }
        active.mash_volume_min_l = Set(input.mash_volume_min_l);
        active.mash_volume_max_l = Set(input.mash_volume_max_l);
        active.sparge_volume_min_l = Set(input.sparge_volume_min_l);
        active.sparge_volume_max_l = Set(input.sparge_volume_max_l);
        if let Some(v) = input.calc_strike_water_temp {
            active.calc_strike_water_temp = Set(v as i32);
        }
        active.updated_at = Set(now_secs() as i32);
        active.update(self.db).await?;
        self.get(id).await
    }

    pub async fn copy(&self, id: &str) -> Result<EquipmentProfile, AppError> {
        let source = equipment_profiles::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;
        let new_id = new_id();
        let now = now_secs() as i32;
        let mut active: equipment_profiles::ActiveModel = source.into();
        active.id = Set(new_id.clone());
        active.name = Set({
            let current = active.name.take().unwrap_or_default();
            format!("{current} (copy)")
        });
        active.created_at = Set(now);
        active.updated_at = Set(now);
        active.insert(self.db).await?;
        self.get(&new_id).await
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
            altitude_adjustment: None,
            aroma_hop_utilization_pct: None,
            batch_size_l: 23.0,
            batch_volume_target: None,
            boil_size_l: 30.0,
            boil_temp_f: None,
            boil_time_min: Some(60.0),
            calc_aroma_hop_utilization: None,
            calc_mash_efficiency: None,
            calc_strike_water_temp: None,
            cooling_shrinkage_pct: None,
            efficiency_pct: 72.0,
            evap_rate_pct_hr: Some(10.0),
            fermenter_loss_l: Some(1.0),
            hlt_deadspace_l: None,
            mash_efficiency_pct: None,
            mash_tun_loss_l: None,
            mash_volume_max_l: None,
            mash_volume_min_l: None,
            name: "10 Gallon Kettle".into(),
            notes: None,
            sparge_method: None,
            sparge_volume_max_l: None,
            sparge_volume_min_l: None,
            trub_chiller_loss_l: Some(1.5),
            whirlpool_time_min: None,
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
