pub mod addition;
pub mod equipment;
pub mod fermentable;
pub mod library;
pub mod mash;
pub mod recipe;
pub mod settings;

use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

pub(crate) fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub(crate) fn new_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub(crate) fn to_dec(v: f64) -> Decimal {
    Decimal::from_f64(v).unwrap_or_default()
}

pub(crate) fn to_dec_opt(v: Option<f64>) -> Option<Decimal> {
    v.map(|x| Decimal::from_f64(x).unwrap_or_default())
}

pub(crate) fn from_dec(v: rust_decimal::Decimal) -> Result<f64, crate::error::AppError> {
    use rust_decimal::prelude::ToPrimitive;
    v.to_f64()
        .ok_or_else(|| crate::error::AppError::Conversion(format!("cannot convert {} to f64", v)))
}

pub(crate) fn from_dec_opt(v: Option<rust_decimal::Decimal>) -> Result<Option<f64>, crate::error::AppError> {
    match v {
        Some(dec) => Ok(Some(from_dec(dec)?)),
        None => Ok(None),
    }
}
