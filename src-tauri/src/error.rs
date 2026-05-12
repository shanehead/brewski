#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("conversion error: {0}")]
    Conversion(String),
    #[error("not found")]
    NotFound,
    #[error("internal error: {0}")]
    Internal(String),
}

impl serde::Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_to_string() {
        assert_eq!(
            serde_json::to_string(&AppError::NotFound).unwrap(),
            "\"not found\""
        );
        assert_eq!(
            serde_json::to_string(&AppError::Conversion("bad value".into())).unwrap(),
            "\"conversion error: bad value\""
        );
    }
}
