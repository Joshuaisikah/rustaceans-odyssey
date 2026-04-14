use std::collections::HashMap;

// ─── Value ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{i}"),
            Value::Float(fl)  => write!(f, "{fl}"),
            Value::Text(s)    => write!(f, "{s}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Null       => write!(f, "NULL"),
        }
    }
}

// ─── Row ─────────────────────────────────────────────────────────────────────
//
// A map of column name → Value.
// Hint: wrap a HashMap<String, Value>.

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub fields: HashMap<String, Value>,
}

impl Row {
    pub fn new() -> Self {
        todo!()
    }

    /// Insert or overwrite a column value.
    pub fn set(&mut self, col: &str, val: Value) {
        todo!()
    }

    /// Return a reference to the value for `col`, or None if absent.
    pub fn get(&self, col: &str) -> Option<&Value> {
        todo!()
    }

    /// True if the row contains the column.
    pub fn has(&self, col: &str) -> bool {
        todo!()
    }

    /// Number of columns set on this row.
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

    // A new row is empty.
    #[test]
    fn test_new_row_is_empty() {
        let r = Row::new();
        assert_eq!(r.column_count(), 0);
    }

    // set() then get() returns the value.
    #[test]
    fn test_set_and_get_integer() {
        let mut r = Row::new();
        r.set("id", Value::Integer(1));
        assert_eq!(r.get("id"), Some(&Value::Integer(1)));
    }

    // get() returns None for a column that was never set.
    #[test]
    fn test_get_missing_column_returns_none() {
        let r = Row::new();
        assert_eq!(r.get("nonexistent"), None);
    }

    // set() overwrites the previous value.
    #[test]
    fn test_set_overwrites_existing_value() {
        let mut r = Row::new();
        r.set("x", Value::Integer(1));
        r.set("x", Value::Integer(99));
        assert_eq!(r.get("x"), Some(&Value::Integer(99)));
    }

    // has() returns true only when the column exists.
    #[test]
    fn test_has_existing_and_missing_column() {
        let mut r = Row::new();
        r.set("name", Value::Text("Alice".into()));
        assert!(r.has("name"));
        assert!(!r.has("age"));
    }

    // column_count tracks the number of distinct columns set.
    #[test]
    fn test_column_count() {
        let mut r = Row::new();
        r.set("a", Value::Integer(1));
        r.set("b", Value::Boolean(true));
        assert_eq!(r.column_count(), 2);
        r.set("a", Value::Null); // overwrite, no new column
        assert_eq!(r.column_count(), 2);
    }

    // Value::Null is a valid value.
    #[test]
    fn test_null_value() {
        let mut r = Row::new();
        r.set("optional", Value::Null);
        assert_eq!(r.get("optional"), Some(&Value::Null));
    }

    // Value display does not panic.
    #[test]
    fn test_value_display() {
        assert_eq!(Value::Integer(7).to_string(), "7");
        assert_eq!(Value::Text("hi".into()).to_string(), "hi");
        assert_eq!(Value::Boolean(true).to_string(), "true");
        assert_eq!(Value::Null.to_string(), "NULL");
    }
}
