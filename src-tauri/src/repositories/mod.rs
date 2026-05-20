pub mod batches;
pub mod equipment;
pub mod fermentable;
pub mod hop;
pub mod ingredient;
pub mod library;
pub mod mash;
pub mod misc;
pub mod recipe;
pub mod recipe_version;
pub mod settings;
pub mod water;
pub mod water_chemistry;
pub mod yeast;

pub(crate) fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub(crate) fn new_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
