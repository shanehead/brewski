-- src-tauri/migrations/003_equipment_profile_enhancements.sql
-- Add new columns for equipment profile enhancements

ALTER TABLE equipment_profiles ADD COLUMN batch_volume_target TEXT NOT NULL DEFAULT 'fermenter';
ALTER TABLE equipment_profiles ADD COLUMN mash_tun_loss_l REAL NOT NULL DEFAULT 0.0;
ALTER TABLE equipment_profiles ADD COLUMN hlt_deadspace_l REAL;
ALTER TABLE equipment_profiles ADD COLUMN cooling_shrinkage_pct REAL NOT NULL DEFAULT 4.0;
ALTER TABLE equipment_profiles ADD COLUMN calc_mash_efficiency INTEGER NOT NULL DEFAULT 1;
ALTER TABLE equipment_profiles ADD COLUMN mash_efficiency_pct REAL;
ALTER TABLE equipment_profiles ADD COLUMN calc_aroma_hop_utilization INTEGER NOT NULL DEFAULT 1;
ALTER TABLE equipment_profiles ADD COLUMN aroma_hop_utilization_pct REAL NOT NULL DEFAULT 23.0;
ALTER TABLE equipment_profiles ADD COLUMN whirlpool_time_min REAL;
ALTER TABLE equipment_profiles ADD COLUMN altitude_adjustment INTEGER NOT NULL DEFAULT 0;
ALTER TABLE equipment_profiles ADD COLUMN boil_temp_f REAL;
ALTER TABLE equipment_profiles ADD COLUMN sparge_method TEXT NOT NULL DEFAULT 'no_sparge';
ALTER TABLE equipment_profiles ADD COLUMN mash_volume_min_l REAL;
ALTER TABLE equipment_profiles ADD COLUMN mash_volume_max_l REAL;
ALTER TABLE equipment_profiles ADD COLUMN sparge_volume_min_l REAL;
ALTER TABLE equipment_profiles ADD COLUMN sparge_volume_max_l REAL;
ALTER TABLE equipment_profiles ADD COLUMN calc_strike_water_temp INTEGER NOT NULL DEFAULT 0;
