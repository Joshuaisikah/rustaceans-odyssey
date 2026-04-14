// ─── database-engine: integration tests ──────────────────────────────────────
//
// Black-box tests through the public API.
// Exercises the full stack: Database → Table → Row → Value.

use database_engine::{Database, DbError, Row, Schema, Value};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn user_schema() -> Schema {
    Schema::new("id", vec!["id", "name", "age"])
}

fn user_row(id: i64, name: &str, age: i64) -> Row {
    let mut r = Row::new();
    r.set("id",   Value::Integer(id));
    r.set("name", Value::Text(name.to_string()));
    r.set("age",  Value::Integer(age));
    r
}

fn by_id(id: i64) -> impl Fn(&Row) -> bool {
    move |r| r.get("id") == Some(&Value::Integer(id))
}

// ── Database lifecycle ────────────────────────────────────────────────────────

#[test]
fn test_fresh_database_is_empty() {
    let db = Database::new();
    assert_eq!(db.table_count(), 0);
}

#[test]
fn test_create_and_drop_table() {
    let mut db = Database::new();
    db.create_table("users", user_schema());
    assert_eq!(db.table_count(), 1);
    assert!(db.get_table("users").is_some());

    assert!(db.drop_table("users"));
    assert_eq!(db.table_count(), 0);
    assert!(db.get_table("users").is_none());
}

#[test]
fn test_drop_nonexistent_table_returns_false() {
    let mut db = Database::new();
    assert!(!db.drop_table("ghost"));
}

#[test]
fn test_table_names_lists_all_tables() {
    let mut db = Database::new();
    db.create_table("users",   user_schema());
    db.create_table("products", Schema::new("id", vec!["id", "price"]));
    let mut names = db.table_names();
    names.sort();
    assert_eq!(names, vec!["products", "users"]);
}

// ── CRUD through Database → Table ────────────────────────────────────────────

#[test]
fn test_insert_and_select_all() {
    let mut db = Database::new();
    db.create_table("users", user_schema());
    let t = db.get_table_mut("users").unwrap();

    t.insert(user_row(1, "Alice", 30)).unwrap();
    let rows = t.select_all();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get("name"), Some(&Value::Text("Alice".into())));
}

#[test]
fn test_insert_multiple_rows_and_count() {
    let mut db = Database::new();
    db.create_table("users", user_schema());
    let t = db.get_table_mut("users").unwrap();

    for i in 1..=5 {
        t.insert(user_row(i, "User", 20 + i)).unwrap();
    }
    assert_eq!(t.count(), 5);
}

#[test]
fn test_duplicate_primary_key_is_rejected() {
    let mut db = Database::new();
    db.create_table("users", user_schema());
    let t = db.get_table_mut("users").unwrap();

    t.insert(user_row(1, "Alice", 30)).unwrap();
    let result = t.insert(user_row(1, "Duplicate", 99));
    assert_eq!(result, Err(DbError::DuplicateKey));
}

#[test]
fn test_missing_primary_key_is_rejected() {
    let mut db = Database::new();
    db.create_table("users", user_schema());
    let t = db.get_table_mut("users").unwrap();

    let mut row = Row::new();
    row.set("name", Value::Text("No ID".into()));
    assert_eq!(t.insert(row), Err(DbError::MissingPrimaryKey));
}

// ── select_where ──────────────────────────────────────────────────────────────

#[test]
fn test_select_where_by_pk() {
    let mut db = Database::new();
    db.create_table("users", user_schema());
    let t = db.get_table_mut("users").unwrap();

    t.insert(user_row(10, "Bob",   25)).unwrap();
    t.insert(user_row(20, "Carol", 35)).unwrap();

    let found = t.select_where(by_id(10));
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].get("name"), Some(&Value::Text("Bob".into())));

    let missing = t.select_where(by_id(99));
    assert!(missing.is_empty());
}

#[test]
fn test_select_where_filters_by_field_value() {
    let mut db = Database::new();
    db.create_table("users", user_schema());
    let t = db.get_table_mut("users").unwrap();

    t.insert(user_row(1, "Alice", 30)).unwrap();
    t.insert(user_row(2, "Bob",   17)).unwrap();
    t.insert(user_row(3, "Carol", 25)).unwrap();

    let adults = t.select_where(|r| {
        matches!(r.get("age"), Some(Value::Integer(a)) if *a >= 18)
    });
    assert_eq!(adults.len(), 2);
}

// ── update_where ─────────────────────────────────────────────────────────────

#[test]
fn test_update_where_mutates_matching_rows() {
    let mut db = Database::new();
    db.create_table("users", user_schema());
    let t = db.get_table_mut("users").unwrap();

    t.insert(user_row(1, "Alice", 30)).unwrap();
    let n = t.update_where(by_id(1), |r| r.set("age", Value::Integer(31)));
    assert_eq!(n, 1);

    let rows = t.select_where(by_id(1));
    assert_eq!(rows[0].get("age"), Some(&Value::Integer(31)));
}

#[test]
fn test_update_where_no_match_returns_zero() {
    let mut db = Database::new();
    db.create_table("users", user_schema());
    let t = db.get_table_mut("users").unwrap();

    let n = t.update_where(by_id(999), |r| r.set("age", Value::Integer(0)));
    assert_eq!(n, 0);
}

// ── delete_where ─────────────────────────────────────────────────────────────

#[test]
fn test_delete_where_removes_matching_rows() {
    let mut db = Database::new();
    db.create_table("users", user_schema());
    let t = db.get_table_mut("users").unwrap();

    t.insert(user_row(1, "Alice", 30)).unwrap();
    t.insert(user_row(2, "Bob",   25)).unwrap();

    let removed = t.delete_where(by_id(1));
    assert_eq!(removed, 1);
    assert_eq!(t.count(), 1);
    assert!(t.select_where(by_id(1)).is_empty());
}

#[test]
fn test_delete_where_no_match_returns_zero() {
    let mut db = Database::new();
    db.create_table("users", user_schema());
    let t = db.get_table_mut("users").unwrap();
    assert_eq!(t.delete_where(by_id(999)), 0);
}

// ── Multi-table independence ──────────────────────────────────────────────────

#[test]
fn test_two_tables_are_independent() {
    let mut db = Database::new();
    db.create_table("a", user_schema());
    db.create_table("b", user_schema());

    db.get_table_mut("a").unwrap().insert(user_row(1, "X", 20)).unwrap();

    assert_eq!(db.get_table("a").unwrap().count(), 1);
    assert_eq!(db.get_table("b").unwrap().count(), 0);
}

// ── Value types ───────────────────────────────────────────────────────────────

#[test]
fn test_null_value_in_row() {
    let schema = Schema::new("id", vec!["id", "optional"]);
    let mut db = Database::new();
    db.create_table("t", schema);
    let t = db.get_table_mut("t").unwrap();

    let mut row = Row::new();
    row.set("id",       Value::Integer(1));
    row.set("optional", Value::Null);
    t.insert(row).unwrap();

    let rows = t.select_where(by_id(1));
    assert_eq!(rows[0].get("optional"), Some(&Value::Null));
}

#[test]
fn test_float_and_bool_values() {
    let schema = Schema::new("id", vec!["id", "score", "active"]);
    let mut db = Database::new();
    db.create_table("t", schema);
    let t = db.get_table_mut("t").unwrap();

    let mut row = Row::new();
    row.set("id",     Value::Integer(1));
    row.set("score",  Value::Float(9.5));
    row.set("active", Value::Boolean(true));
    t.insert(row).unwrap();

    let rows = t.select_where(by_id(1));
    assert_eq!(rows[0].get("score"),  Some(&Value::Float(9.5)));
    assert_eq!(rows[0].get("active"), Some(&Value::Boolean(true)));
}
