-- src-tauri/migrations/014_equipment_profile_parity.sql

-- 1. Replace evap_rate_pct_hr with evap_rate_l_hr (absolute volume per hour)
ALTER TABLE equipment_profiles ADD COLUMN evap_rate_l_hr REAL NOT NULL DEFAULT 3.8;
UPDATE equipment_profiles
  SET evap_rate_l_hr = evap_rate_pct_hr / 100.0 * boil_size_l;
ALTER TABLE equipment_profiles DROP COLUMN evap_rate_pct_hr;

-- 2. Replace tun_weight_kg + tun_specific_heat with single heat-capacity field
ALTER TABLE equipment_profiles ADD COLUMN tun_heat_capacity_l REAL NOT NULL DEFAULT 0.0;
ALTER TABLE equipment_profiles DROP COLUMN tun_weight_kg;
ALTER TABLE equipment_profiles DROP COLUMN tun_specific_heat;

-- 3. New fields
ALTER TABLE equipment_profiles ADD COLUMN hopstand_temp_f REAL NOT NULL DEFAULT 176.0;
ALTER TABLE equipment_profiles ADD COLUMN grain_absorption_rate_l_per_kg REAL NOT NULL DEFAULT 1.04;
ALTER TABLE equipment_profiles ADD COLUMN water_grain_ratio_l_per_kg REAL NOT NULL DEFAULT 3.12;
ALTER TABLE equipment_profiles ADD COLUMN include_grain_volume_in_mash_limits INTEGER NOT NULL DEFAULT 1;
ALTER TABLE equipment_profiles ADD COLUMN overflow_target TEXT NOT NULL DEFAULT 'mash';
ALTER TABLE equipment_profiles ADD COLUMN hlt_water_limit_min_l REAL;
ALTER TABLE equipment_profiles ADD COLUMN room_temp_f REAL NOT NULL DEFAULT 68.0;
ALTER TABLE equipment_profiles ADD COLUMN grain_temp_f REAL NOT NULL DEFAULT 68.0;
ALTER TABLE equipment_profiles ADD COLUMN sparge_temp_f REAL;
