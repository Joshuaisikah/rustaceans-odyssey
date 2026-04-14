// ─── Schema — table structure definition ─────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Schema {
    pub columns:     Vec<String>,
    pub primary_key: String,
}

impl Schema {
    pub fn new(primary_key: &str, columns: Vec<&str>) -> Self {
        Schema {
            primary_key: primary_key.to_string(),
            columns:     columns.into_iter().map(String::from).collect(),
        }
    }

    /// True if `col` is declared in this schema.
    pub fn has_column(&self, col: &str) -> bool {
        self.columns.iter().any(|c| c == col)
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn schema() -> Schema {
        Schema::new("id", vec!["id", "name", "age"])
    }

    #[test]
    fn test_primary_key_stored() {
        assert_eq!(schema().primary_key, "id");
    }

    #[test]
    fn test_column_count() {
        assert_eq!(schema().column_count(), 3);
    }

    #[test]
    fn test_has_column_existing() {
        assert!(schema().has_column("name"));
    }

    #[test]
    fn test_has_column_missing() {
        assert!(!schema().has_column("email"));
    }

    #[test]
    fn test_clone_is_independent() {
        let a = schema();
        let mut b = a.clone();
        b.columns.push("extra".into());
        assert_eq!(a.column_count(), 3);
        assert_eq!(b.column_count(), 4);
    }
}
