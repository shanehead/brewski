//! Deterministic content fingerprint for recipe identity / change detection.
//!
//! Include-by-default: we serialize the whole `Recipe` and then strip the keys
//! that are NOT part of brew identity (metadata + resolved profile *values*).
//! A future recipe field is therefore part of the hash automatically unless it
//! is named like a surrogate key or added to the metadata exclude set.

use crate::error::AppError;
use crate::models::Recipe;
use serde_json::Value;
use sha2::{Digest, Sha256};

/// Bump when the canonical projection changes; stored hashes are prefixed with
/// this so stale ones can be recomputed instead of trusted.
pub const PROJECTION_VERSION: &str = "1";

/// Surrogate keys removed from EVERY object (live rows and snapshot rows use
/// different ids for identical content). Library reference ids like
/// `fermentable_id` / `equipment_profile_id` are deliberately NOT in this set.
const SURROGATE_KEYS: &[&str] = &["id", "recipe_id", "mash_id"];

/// Top-level keys excluded from identity: metadata + the resolved profile value
/// objects. The profile *selection* ids (`equipment_profile_id`,
/// `mash_water_id`, `sparge_water_id`) are kept.
const TOP_EXCLUDE_KEYS: &[&str] = &[
    "created_at",
    "updated_at",
    "source",
    "taste_notes",
    "taste_rating",
    "image_path",
    "name",
    "brewer",
    "asst_brewer",
    "date",
    "style",             // resolved style object
    "style_id",          // descriptive classification, not brew content
    "equipment_profile", // resolved profile values (selection id kept)
];

fn scrub_surrogates(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for k in SURROGATE_KEYS {
                map.remove(*k);
            }
            for (_, v) in map.iter_mut() {
                scrub_surrogates(v);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                scrub_surrogates(v);
            }
        }
        _ => {}
    }
}

/// Returns the canonical JSON bytes used for hashing. Exposed for tests.
pub fn canonical_bytes(recipe: &Recipe) -> Result<Vec<u8>, AppError> {
    let mut value = serde_json::to_value(recipe).map_err(|e| AppError::Internal(e.to_string()))?;
    scrub_surrogates(&mut value);
    if let Value::Object(map) = &mut value {
        for k in TOP_EXCLUDE_KEYS {
            map.remove(*k);
        }
    }
    serde_json::to_vec(&value).map_err(|e| AppError::Internal(e.to_string()))
}

/// `"<projection>:<hex sha256>"` fingerprint of a recipe's brew identity.
pub fn recipe_content_hash(recipe: &Recipe) -> Result<String, AppError> {
    let bytes = canonical_bytes(recipe)?;
    let digest = Sha256::digest(&bytes);
    Ok(format!("{PROJECTION_VERSION}:{digest:x}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Recipe;

    fn recipe_from_json(v: serde_json::Value) -> Recipe {
        serde_json::from_value(v).expect("valid Recipe json")
    }

    fn base_json() -> serde_json::Value {
        // Raw string keeps the whole fixture in one literal (no json! macro
        // recursion limit) and avoids a brittle two-map merge. "type_" matches
        // the generated serde key for the renamed `type` column.
        serde_json::from_str::<serde_json::Value>(
            r#"{
                "id": "r1",
                "name": "Test IPA",
                "type_": "All Grain",
                "batch_size_l": 23.0,
                "boil_size_l": 27.0,
                "boil_time_min": 60.0,
                "efficiency_pct": 72.0,
                "fermentation_stages": 1,
                "forced_carbonation": false,
                "hopstand_temp_c": 80.0,
                "created_at": 1,
                "updated_at": 1,
                "source": "user",
                "brewer": null,
                "asst_brewer": null,
                "style_id": null,
                "equipment_profile_id": "eq1",
                "notes": null,
                "taste_notes": null,
                "taste_rating": null,
                "og": null,
                "fg": null,
                "primary_age_days": null,
                "primary_temp_c": null,
                "secondary_age_days": null,
                "secondary_temp_c": null,
                "tertiary_age_days": null,
                "tertiary_temp_c": null,
                "age_days": null,
                "age_temp_c": null,
                "carbonation_vols": null,
                "priming_sugar_name": null,
                "carbonation_temp_c": null,
                "priming_sugar_equiv": null,
                "keg_priming_factor": null,
                "date": null,
                "mash_water_id": null,
                "sparge_water_id": null,
                "image_path": null,
                "style": null,
                "equipment_profile": null,
                "fermentables": [],
                "hops": [],
                "yeasts": [],
                "miscs": [],
                "waters": [],
                "water_adjustments": [],
                "mash": null
            }"#,
        )
        .expect("valid base recipe json")
    }

    /// Builds a recipe fixture whose `mash` + `steps` carry the given surrogate
    /// ids but identical real content, so two calls differing only in ids must
    /// hash the same.
    fn json_with_mash(
        mash_id: &str,
        mash_recipe_id: &str,
        step_id: &str,
        step_mash_id: &str,
    ) -> serde_json::Value {
        let mut j = base_json();
        j["mash"] = serde_json::json!({
            "id": mash_id,
            "recipe_id": mash_recipe_id,
            "name": "Single Infusion, Medium Body",
            "equip_adjust": true,
            "grain_temp_c": 20.0,
            "notes": null,
            "ph": null,
            "ratio_l_per_kg": 2.8,
            "sparge_temp_c": 75.6,
            "tun_specific_heat": null,
            "tun_temp_c": null,
            "tun_weight_kg": null,
            "steps": [
                {
                    "id": step_id,
                    "mash_id": step_mash_id,
                    "name": "Saccharification",
                    "type_": "Infusion",
                    "step_order": 1,
                    "step_temp_c": 67.0,
                    "step_time_min": 60,
                    "end_temp_c": null,
                    "infuse_amount_l": 14.0,
                    "ramp_time_min": null
                }
            ]
        });
        j
    }

    #[test]
    fn identical_content_hashes_equal() {
        let a = recipe_from_json(base_json());
        let b = recipe_from_json(base_json());
        assert_eq!(
            recipe_content_hash(&a).unwrap(),
            recipe_content_hash(&b).unwrap()
        );
    }

    #[test]
    fn surrogate_ids_do_not_affect_hash() {
        let a = recipe_from_json(base_json());
        let mut j = base_json();
        j["id"] = serde_json::json!("DIFFERENT");
        j["created_at"] = serde_json::json!(999);
        let b = recipe_from_json(j);
        assert_eq!(
            recipe_content_hash(&a).unwrap(),
            recipe_content_hash(&b).unwrap()
        );
    }

    #[test]
    fn excluded_metadata_does_not_affect_hash() {
        let a = recipe_from_json(base_json());
        let mut j = base_json();
        j["name"] = serde_json::json!("Renamed");
        j["taste_notes"] = serde_json::json!("delicious");
        j["image_path"] = serde_json::json!("/x.png");
        let b = recipe_from_json(j);
        assert_eq!(
            recipe_content_hash(&a).unwrap(),
            recipe_content_hash(&b).unwrap()
        );
    }

    #[test]
    fn equipment_profile_selection_changes_hash() {
        let a = recipe_from_json(base_json());
        let mut j = base_json();
        j["equipment_profile_id"] = serde_json::json!("eq2");
        let b = recipe_from_json(j);
        assert_ne!(
            recipe_content_hash(&a).unwrap(),
            recipe_content_hash(&b).unwrap()
        );
    }

    #[test]
    fn brew_field_changes_hash() {
        let a = recipe_from_json(base_json());
        let mut j = base_json();
        j["batch_size_l"] = serde_json::json!(20.0);
        let b = recipe_from_json(j);
        assert_ne!(
            recipe_content_hash(&a).unwrap(),
            recipe_content_hash(&b).unwrap()
        );
    }

    #[test]
    fn mash_surrogate_ids_do_not_affect_hash() {
        // Same mash content, different surrogate ids on the mash and its step.
        let a = recipe_from_json(json_with_mash("m1", "r1", "s1", "m1"));
        let b = recipe_from_json(json_with_mash(
            "DIFFERENT_MASH",
            "DIFFERENT_RECIPE",
            "DIFFERENT_STEP",
            "DIFFERENT_MASH",
        ));
        assert_eq!(
            recipe_content_hash(&a).unwrap(),
            recipe_content_hash(&b).unwrap()
        );
    }

    #[test]
    fn hash_has_projection_prefix() {
        let a = recipe_from_json(base_json());
        assert!(recipe_content_hash(&a).unwrap().starts_with("1:"));
    }
}
