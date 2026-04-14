use crate::error::DbError;
use crate::types::{Row, Schema, Value};

// ─── Table — in-memory relational storage ─────────────────────────────────────
//
// Stores rows keyed by the primary key field defined in Schema.
//
// Hint: Vec<Row> for ordered storage, or HashMap<Value, Row> for O(1) PK lookup.

pub struct Table {
    pub name:   String,
    pub schema: Schema,
    // TODO: internal storage (Vec<Row> or HashMap<String/Value, Row>)
}

impl Table {
    pub fn new(name: &str, schema: Schema) -> Self {
        todo!()
    }

    /// Insert a row.
    ///
    /// - Returns `Err(MissingPrimaryKey)` if the PK field is absent.
    /// - Returns `Err(DuplicateKey)` if a row with the same PK already exists.
    pub fn insert(&mut self, row: Row) -> Result<(), DbError> {
        todo!()
    }

    /// Return all rows in insertion order.
    pub fn select_all(&self) -> Vec<&Row> {
        todo!()
    }

    /// Return rows for which `predicate` returns true.
    pub fn select_where<F: Fn(&Row) -> bool>(&self, predicate: F) -> Vec<&Row> {
        todo!()
    }

    /// Delete rows matching `predicate`. Returns the count removed.
    pub fn delete_where<F: Fn(&Row) -> bool>(&mut self, predicate: F) -> usize {
        todo!()
    }

    /// Mutate rows matching `predicate` via `updater`. Returns count modified.
    pub fn update_where<F, U>(&mut self, predicate: F, updater: U) -> usize
    where
        F: Fn(&Row) -> bool,
        U: Fn(&mut Row),
    {
        todo!()
    }

    pub fn count(&self) -> usize {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn schema() -> Schema { Schema::new("id", vec!["id", "name", "age"]) }

    fn row(id: i64, name: &str, age: i64) -> Row {
        let mut r = Row::new();
        r.set("id",   Value::Integer(id));
        r.set("name", Value::Text(name.into()));
        r.set("age",  Value::Integer(age));
        r
    }

    #[test]
    fn test_insert_increases_count() {
        let mut t = Table::new("t", schema());
        t.insert(row(1, "Alice", 30)).unwrap();
        assert_eq!(t.count(), 1);
    }

    #[test]
    fn test_select_all_returns_all() {
        let mut t = Table::new("t", schema());
        t.insert(row(1, "A", 20)).unwrap();
        t.insert(row(2, "B", 25)).unwrap();
        assert_eq!(t.select_all().len(), 2);
    }

    #[test]
    fn test_duplicate_pk_rejected() {
        let mut t = Table::new("t", schema());
        t.insert(row(1, "Alice", 30)).unwrap();
        assert_eq!(t.insert(row(1, "Alice2", 31)), Err(DbError::DuplicateKey));
    }

    #[test]
    fn test_missing_pk_rejected() {
        let mut t = Table::new("t", schema());
        let mut r = Row::new();
        r.set("name", Value::Text("No ID".into()));
        assert_eq!(t.insert(r), Err(DbError::MissingPrimaryKey));
    }

    #[test]
    fn test_select_where_filters() {
        let mut t = Table::new("t", schema());
        t.insert(row(1, "Alice", 30)).unwrap();
        t.insert(row(2, "Bob",   20)).unwrap();
        let adults = t.select_where(|r| matches!(r.get("age"), Some(Value::Integer(a)) if *a >= 21));
        assert_eq!(adults.len(), 1);
    }

    #[test]
    fn test_delete_where_removes_and_counts() {
        let mut t = Table::new("t", schema());
        t.insert(row(1, "Alice", 30)).unwrap();
        t.insert(row(2, "Bob",   25)).unwrap();
        let del = t.delete_where(|r| r.get("name") == Some(&Value::Text("Bob".into())));
        assert_eq!(del, 1);
        assert_eq!(t.count(), 1);
    }

    #[test]
    fn test_update_where_mutates() {
        let mut t = Table::new("t", schema());
        t.insert(row(1, "Alice", 30)).unwrap();
        let n = t.update_where(
            |r| r.get("name") == Some(&Value::Text("Alice".into())),
            |r| r.set("age", Value::Integer(31)),
        );
        assert_eq!(n, 1);
        let alice = t.select_where(|r| r.get("name") == Some(&Value::Text("Alice".into())));
        assert_eq!(alice[0].get("age"), Some(&Value::Integer(31)));
    }
}
