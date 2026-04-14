use std::collections::HashMap;
use super::value::Value;

// ─── Row — a single record (column → value map) ───────────────────────────────
//
// Hint: wrap a HashMap<String, Value>.

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub fields: HashMap<String, Value>,
}

impl Row {
    pub fn new() -> Self {
        todo!()
    }

    /// Insert or overwrite `col` with `val`.
    pub fn set(&mut self, col: &str, val: Value) {
        todo!()
    }

    /// Return a reference to the value for `col`, or None.
    pub fn get(&self, col: &str) -> Option<&Value> {
        todo!()
    }

    /// True if this row has a value for `col`.
    pub fn has(&self, col: &str) -> bool {
        todo!()
    }

    /// Number of distinct columns set on this row.
    pub fn column_count(&self) -> usize {
        todo!()
    }
}

impl Default for Row {
    fn default() -> Self { Self::new() }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_row_is_empty() {
        let r = Row::new();
        assert_eq!(r.column_count(), 0);
    }

    #[test]
    fn test_set_and_get() {
        let mut r = Row::new();
        r.set("id", Value::Integer(1));
        assert_eq!(r.get("id"), Some(&Value::Integer(1)));
    }

    #[test]
    fn test_get_missing_returns_none() {
        let r = Row::new();
        assert_eq!(r.get("ghost"), None);
    }

    #[test]
    fn test_set_overwrites() {
        let mut r = Row::new();
        r.set("x", Value::Integer(1));
        r.set("x", Value::Integer(99));
        assert_eq!(r.get("x"), Some(&Value::Integer(99)));
        assert_eq!(r.column_count(), 1);
    }

    #[test]
    fn test_has_existing_and_missing() {
        let mut r = Row::new();
        r.set("name", Value::Text("Alice".into()));
        assert!(r.has("name"));
        assert!(!r.has("age"));
    }

    #[test]
    fn test_null_value_stored() {
        let mut r = Row::new();
        r.set("optional", Value::Null);
        assert_eq!(r.get("optional"), Some(&Value::Null));
    }
}
