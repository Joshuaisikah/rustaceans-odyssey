use crate::storage::Table;
use crate::types::Schema;

// ─── Database ────────────────────────────────────────────────────────────────
//
// A collection of named tables.
// Hint: use a HashMap<String, Table>.

pub struct Database {
    // TODO: HashMap<String, Table>
}

impl Database {
    pub fn new() -> Self {
        todo!()
    }

    /// Create a table. Silently replaces any existing table with the same name.
    pub fn create_table(&mut self, name: &str, schema: Schema) {
        todo!()
    }

    /// Return an immutable reference to a table, or None if it doesn't exist.
    pub fn get_table(&self, name: &str) -> Option<&Table> {
        todo!()
    }

    /// Return a mutable reference to a table, or None if it doesn't exist.
    pub fn get_table_mut(&mut self, name: &str) -> Option<&mut Table> {
        todo!()
    }

    /// Remove a table. Returns true if it existed, false otherwise.
    pub fn drop_table(&mut self, name: &str) -> bool {
        todo!()
    }

    /// Names of all tables currently in the database.
    pub fn table_names(&self) -> Vec<&str> {
        todo!()
    }

    /// Total number of tables.
    pub fn table_count(&self) -> usize {
        todo!()
    }
}

impl Default for Database {
    fn default() -> Self { Self::new() }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Row, Value};

    fn schema(pk: &str) -> Schema {
        Schema::new(pk, vec![pk, "name"])
    }

    // A fresh database has no tables.
    #[test]
    fn test_new_database_is_empty() {
        let db = Database::new();
        assert_eq!(db.table_count(), 0);
    }

    // create_table makes the table reachable via get_table.
    #[test]
    fn test_create_and_get_table() {
        let mut db = Database::new();
        db.create_table("users", schema("id"));
        assert!(db.get_table("users").is_some());
    }

    // get_table on a missing name returns None.
    #[test]
    fn test_get_missing_table_returns_none() {
        let db = Database::new();
        assert!(db.get_table("ghost").is_none());
    }

    // Multiple tables are independent.
    #[test]
    fn test_multiple_independent_tables() {
        let mut db = Database::new();
        db.create_table("a", schema("id"));
        db.create_table("b", schema("id"));

        let mut row = Row::new();
        row.set("id",   Value::Integer(1));
        row.set("name", Value::Text("X".into()));
        db.get_table_mut("a").unwrap().insert(row).unwrap();

        assert_eq!(db.get_table("a").unwrap().count(), 1);
        assert_eq!(db.get_table("b").unwrap().count(), 0);
    }

    // drop_table returns true and removes the table.
    #[test]
    fn test_drop_existing_table() {
        let mut db = Database::new();
        db.create_table("tmp", schema("id"));
        assert!(db.drop_table("tmp"));
        assert!(db.get_table("tmp").is_none());
    }

    // drop_table on a nonexistent table returns false.
    #[test]
    fn test_drop_nonexistent_table_returns_false() {
        let mut db = Database::new();
        assert!(!db.drop_table("ghost"));
    }

    // table_names lists all tables.
    #[test]
    fn test_table_names_lists_all() {
        let mut db = Database::new();
        db.create_table("x", schema("id"));
        db.create_table("y", schema("id"));
        let mut names = db.table_names();
        names.sort();
        assert_eq!(names, vec!["x", "y"]);
    }

    // table_count tracks creation and deletion.
    #[test]
    fn test_table_count_tracks_changes() {
        let mut db = Database::new();
        db.create_table("a", schema("id"));
        assert_eq!(db.table_count(), 1);
        db.drop_table("a");
        assert_eq!(db.table_count(), 0);
    }
}
