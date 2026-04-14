use crate::error::DbError;
use crate::row::{Row, Value};

// ─── Schema ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Schema {
    pub columns: Vec<String>,
    pub primary_key: String,
}

impl Schema {
    pub fn new(primary_key: &str, columns: Vec<&str>) -> Self {
        Schema {
            primary_key: primary_key.to_string(),
            columns: columns.into_iter().map(String::from).collect(),
        }
    }
}

// ─── Table ───────────────────────────────────────────────────────────────────
//
// In-memory table: insert, select, update, delete.
// Hint: store rows in a Vec<Row> (or HashMap keyed by the PK value).

pub struct Table {
    pub name: String,
    pub schema: Schema,
    // TODO: add internal storage
}

impl Table {
    pub fn new(name: &str, schema: Schema) -> Self {
        todo!()
    }

    /// Insert a row.
    /// Returns Err(MissingPrimaryKey) when the PK field is absent.
    /// Returns Err(DuplicateKey) when a row with the same PK already exists.
    pub fn insert(&mut self, row: Row) -> Result<(), DbError> {
        todo!()
    }

    /// Return every row in insertion order.
    pub fn select_all(&self) -> Vec<&Row> {
        todo!()
    }

    /// Return rows satisfying `predicate`.
    pub fn select_where<F: Fn(&Row) -> bool>(&self, predicate: F) -> Vec<&Row> {
        todo!()
    }

    /// Delete rows matching `predicate`. Returns the number of rows removed.
    pub fn delete_where<F: Fn(&Row) -> bool>(&mut self, predicate: F) -> usize {
        todo!()
    }

    /// Apply `updater` to every row matching `predicate`.
    /// Returns the number of rows mutated.
    pub fn update_where<F, U>(&mut self, predicate: F, updater: U) -> usize
    where
        F: Fn(&Row) -> bool,
        U: Fn(&mut Row),
    {
        todo!()
    }

    /// Total number of rows currently in the table.
    pub fn count(&self) -> usize {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn schema() -> Schema {
        Schema::new("id", vec!["id", "name", "age"])
    }

    fn row(id: i64, name: &str, age: i64) -> Row {
        let mut r = Row::new();
        r.set("id",   Value::Integer(id));
        r.set("name", Value::Text(name.to_string()));
        r.set("age",  Value::Integer(age));
        r
    }

    // Inserting a row and counting it.
    #[test]
    fn test_insert_increases_count() {
        let mut t = Table::new("users", schema());
        t.insert(row(1, "Alice", 30)).unwrap();
        assert_eq!(t.count(), 1);
    }

    // select_all returns every row.
    #[test]
    fn test_select_all_returns_all_rows() {
        let mut t = Table::new("users", schema());
        t.insert(row(1, "Alice", 30)).unwrap();
        t.insert(row(2, "Bob",   25)).unwrap();
        assert_eq!(t.select_all().len(), 2);
    }

    // Duplicate PK returns DuplicateKey.
    #[test]
    fn test_insert_duplicate_pk_returns_error() {
        let mut t = Table::new("users", schema());
        t.insert(row(1, "Alice", 30)).unwrap();
        assert_eq!(t.insert(row(1, "Alice2", 31)), Err(DbError::DuplicateKey));
    }

    // Missing PK field returns MissingPrimaryKey.
    #[test]
    fn test_insert_missing_pk_returns_error() {
        let mut t = Table::new("users", schema());
        let mut r = Row::new();
        r.set("name", Value::Text("No ID".into()));
        assert_eq!(t.insert(r), Err(DbError::MissingPrimaryKey));
    }

    // select_where filters by predicate.
    #[test]
    fn test_select_where_filters() {
        let mut t = Table::new("users", schema());
        t.insert(row(1, "Alice", 30)).unwrap();
        t.insert(row(2, "Bob",   20)).unwrap();
        t.insert(row(3, "Carol", 35)).unwrap();

        let adults = t.select_where(|r| {
            matches!(r.get("age"), Some(Value::Integer(a)) if *a >= 30)
        });
        assert_eq!(adults.len(), 2);
    }

    // select_where with no matches returns empty.
    #[test]
    fn test_select_where_no_match_returns_empty() {
        let mut t = Table::new("users", schema());
        t.insert(row(1, "Alice", 30)).unwrap();
        assert!(t.select_where(|r| r.get("name") == Some(&Value::Text("Nobody".into()))).is_empty());
    }

    // delete_where removes matching rows and reports count.
    #[test]
    fn test_delete_where_removes_correct_rows() {
        let mut t = Table::new("users", schema());
        t.insert(row(1, "Alice", 30)).unwrap();
        t.insert(row(2, "Bob",   25)).unwrap();

        let deleted = t.delete_where(|r| r.get("name") == Some(&Value::Text("Bob".into())));
        assert_eq!(deleted, 1);
        assert_eq!(t.count(), 1);
    }

    // update_where mutates matching rows only.
    #[test]
    fn test_update_where_mutates_correctly() {
        let mut t = Table::new("users", schema());
        t.insert(row(1, "Alice", 30)).unwrap();
        t.insert(row(2, "Bob",   25)).unwrap();

        let updated = t.update_where(
            |r| r.get("name") == Some(&Value::Text("Alice".into())),
            |r| r.set("age", Value::Integer(31)),
        );
        assert_eq!(updated, 1);

        let alice = t.select_where(|r| r.get("name") == Some(&Value::Text("Alice".into())));
        assert_eq!(alice[0].get("age"), Some(&Value::Integer(31)));
    }

    // count() returns 0 on an empty table.
    #[test]
    fn test_empty_table_count_is_zero() {
        let t = Table::new("empty", schema());
        assert_eq!(t.count(), 0);
    }
}
