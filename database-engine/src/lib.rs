use std::collections::HashMap;

// ─── Value type ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    Null,
}

// ─── Row ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub fields: HashMap<String, Value>,
}

impl Row {
    pub fn new() -> Self {
        todo!()
    }

    pub fn set(&mut self, col: &str, val: Value) {
        todo!()
    }

    pub fn get(&self, col: &str) -> Option<&Value> {
        todo!()
    }
}

// ─── Schema ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Schema {
    pub columns: Vec<String>,
    pub primary_key: String,
}

// ─── Errors ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum DbError {
    MissingPrimaryKey,
    DuplicateKey,
    TableNotFound,
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::MissingPrimaryKey => write!(f, "missing primary key"),
            DbError::DuplicateKey => write!(f, "duplicate key"),
            DbError::TableNotFound => write!(f, "table not found"),
        }
    }
}

// ─── Table ───────────────────────────────────────────────────────────────────

pub struct Table {
    pub name: String,
    pub schema: Schema,
    // TODO: add internal storage
}

impl Table {
    pub fn new(name: &str, schema: Schema) -> Self {
        todo!()
    }

    /// Insert a row. Returns `Err(DuplicateKey)` when the PK already exists.
    /// Returns `Err(MissingPrimaryKey)` when the row has no PK field.
    pub fn insert(&mut self, row: Row) -> Result<(), DbError> {
        todo!()
    }

    /// Return every row in the table.
    pub fn select_all(&self) -> Vec<&Row> {
        todo!()
    }

    /// Return rows matching the predicate.
    pub fn select_where<F: Fn(&Row) -> bool>(&self, predicate: F) -> Vec<&Row> {
        todo!()
    }

    /// Delete rows matching the predicate. Returns the number of rows removed.
    pub fn delete_where<F: Fn(&Row) -> bool>(&mut self, predicate: F) -> usize {
        todo!()
    }

    /// Apply `updater` to every row matching `predicate`. Returns count updated.
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

// ─── Database ────────────────────────────────────────────────────────────────

pub struct Database {
    // TODO: add your fields
}

impl Database {
    pub fn new() -> Self {
        todo!()
    }

    pub fn create_table(&mut self, name: &str, schema: Schema) {
        todo!()
    }

    pub fn get_table(&self, name: &str) -> Option<&Table> {
        todo!()
    }

    pub fn get_table_mut(&mut self, name: &str) -> Option<&mut Table> {
        todo!()
    }

    pub fn drop_table(&mut self, name: &str) -> bool {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_schema(pk: &str) -> Schema {
        Schema {
            columns: vec!["id".into(), "name".into(), "age".into()],
            primary_key: pk.to_string(),
        }
    }

    fn make_row(id: i64, name: &str, age: i64) -> Row {
        let mut r = Row::new();
        r.set("id", Value::Integer(id));
        r.set("name", Value::Text(name.to_string()));
        r.set("age", Value::Integer(age));
        r
    }

    // A row can be inserted and retrieved.
    #[test]
    fn test_insert_and_select_all() {
        let mut t = Table::new("users", make_schema("id"));
        t.insert(make_row(1, "Alice", 30)).unwrap();
        t.insert(make_row(2, "Bob", 25)).unwrap();
        assert_eq!(t.count(), 2);
        let rows = t.select_all();
        assert_eq!(rows.len(), 2);
    }

    // Inserting a row with a duplicate primary key returns DuplicateKey.
    #[test]
    fn test_insert_duplicate_key_returns_error() {
        let mut t = Table::new("users", make_schema("id"));
        t.insert(make_row(1, "Alice", 30)).unwrap();
        let err = t.insert(make_row(1, "Alice2", 31));
        assert_eq!(err, Err(DbError::DuplicateKey));
    }

    // Inserting a row without the PK field returns MissingPrimaryKey.
    #[test]
    fn test_insert_missing_pk_returns_error() {
        let mut t = Table::new("users", make_schema("id"));
        let mut r = Row::new();
        r.set("name", Value::Text("No ID".into()));
        let err = t.insert(r);
        assert_eq!(err, Err(DbError::MissingPrimaryKey));
    }

    // select_where filters by predicate.
    #[test]
    fn test_select_where_filters_rows() {
        let mut t = Table::new("users", make_schema("id"));
        t.insert(make_row(1, "Alice", 30)).unwrap();
        t.insert(make_row(2, "Bob", 20)).unwrap();
        t.insert(make_row(3, "Carol", 35)).unwrap();

        let adults: Vec<_> = t.select_where(|r| {
            matches!(r.get("age"), Some(Value::Integer(a)) if *a >= 30)
        });
        assert_eq!(adults.len(), 2);
    }

    // delete_where removes matching rows and returns the count.
    #[test]
    fn test_delete_where_removes_and_returns_count() {
        let mut t = Table::new("users", make_schema("id"));
        t.insert(make_row(1, "Alice", 30)).unwrap();
        t.insert(make_row(2, "Bob", 20)).unwrap();

        let deleted = t.delete_where(|r| r.get("name") == Some(&Value::Text("Bob".into())));
        assert_eq!(deleted, 1);
        assert_eq!(t.count(), 1);
    }

    // update_where mutates matching rows.
    #[test]
    fn test_update_where_mutates_matching_rows() {
        let mut t = Table::new("users", make_schema("id"));
        t.insert(make_row(1, "Alice", 30)).unwrap();
        t.insert(make_row(2, "Bob", 20)).unwrap();

        let updated = t.update_where(
            |r| r.get("name") == Some(&Value::Text("Alice".into())),
            |r| r.set("age", Value::Integer(31)),
        );
        assert_eq!(updated, 1);

        let alice_rows = t.select_where(|r| r.get("name") == Some(&Value::Text("Alice".into())));
        assert_eq!(alice_rows[0].get("age"), Some(&Value::Integer(31)));
    }

    // select_where with no matches returns an empty vec.
    #[test]
    fn test_select_where_no_matches_returns_empty() {
        let mut t = Table::new("users", make_schema("id"));
        t.insert(make_row(1, "Alice", 30)).unwrap();
        let results = t.select_where(|r| r.get("name") == Some(&Value::Text("Nobody".into())));
        assert!(results.is_empty());
    }

    // Database can hold multiple independent tables.
    #[test]
    fn test_database_multiple_tables() {
        let mut db = Database::new();
        db.create_table("users", make_schema("id"));
        db.create_table("products", Schema {
            columns: vec!["sku".into(), "price".into()],
            primary_key: "sku".into(),
        });

        let users = db.get_table_mut("users").unwrap();
        users.insert(make_row(1, "Alice", 30)).unwrap();

        assert_eq!(db.get_table("users").unwrap().count(), 1);
        assert_eq!(db.get_table("products").unwrap().count(), 0);
    }

    // drop_table returns true and makes the table unavailable.
    #[test]
    fn test_drop_table() {
        let mut db = Database::new();
        db.create_table("tmp", make_schema("id"));
        assert!(db.drop_table("tmp"));
        assert!(db.get_table("tmp").is_none());
    }

    // drop_table on a non-existent name returns false.
    #[test]
    fn test_drop_nonexistent_table_returns_false() {
        let mut db = Database::new();
        assert!(!db.drop_table("ghost"));
    }

    // Row::get returns None for a column that was never set.
    #[test]
    fn test_row_get_missing_column() {
        let r = Row::new();
        assert_eq!(r.get("nonexistent"), None);
    }
}
