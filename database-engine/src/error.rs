// ─── DbError ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum DbError {
    MissingPrimaryKey,
    DuplicateKey,
    TableNotFound,
    ColumnNotFound(String),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::MissingPrimaryKey      => write!(f, "missing primary key"),
            DbError::DuplicateKey           => write!(f, "duplicate key"),
            DbError::TableNotFound          => write!(f, "table not found"),
            DbError::ColumnNotFound(col)    => write!(f, "column not found: {col}"),
        }
    }
}

impl std::error::Error for DbError {}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Display must not panic for any variant.
    #[test]
    fn test_display_missing_primary_key() {
        assert_eq!(DbError::MissingPrimaryKey.to_string(), "missing primary key");
    }

    #[test]
    fn test_display_duplicate_key() {
        assert_eq!(DbError::DuplicateKey.to_string(), "duplicate key");
    }

    #[test]
    fn test_display_table_not_found() {
        assert_eq!(DbError::TableNotFound.to_string(), "table not found");
    }

    #[test]
    fn test_display_column_not_found() {
        let e = DbError::ColumnNotFound("age".to_string());
        assert!(e.to_string().contains("age"));
    }

    // PartialEq must hold.
    #[test]
    fn test_equality() {
        assert_eq!(DbError::DuplicateKey, DbError::DuplicateKey);
        assert_ne!(DbError::DuplicateKey, DbError::TableNotFound);
    }

    // DbError is usable in a Result chain.
    #[test]
    fn test_usable_as_result_err() {
        fn fail() -> Result<(), DbError> { Err(DbError::MissingPrimaryKey) }
        assert!(fail().is_err());
    }
}
