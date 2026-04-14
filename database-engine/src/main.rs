// ─── database-engine — integration demo ──────────────────────────────────────
//
// Exercises the full stack: Database → Table → Row → Value.
// Run with:  cargo run -p database-engine

use database_engine::{Database, DbError, Row, Schema, Value};

fn main() {
    println!("=== database-engine integration demo ===\n");

    demo_basic_crud();
    demo_duplicate_key_rejection();
    demo_filtered_queries();
    demo_multi_table_database();

    println!("\nAll demos completed.");
}

// ── Demo 1: basic CRUD ────────────────────────────────────────────────────────
fn demo_basic_crud() {
    println!("[ Demo 1 ] basic CRUD");

    let mut db = Database::new();
    db.create_table("users", Schema::new("id", vec!["id", "name", "age"]));

    let t = db.get_table_mut("users").unwrap();

    // INSERT
    let mut r1 = Row::new();
    r1.set("id",   Value::Integer(1));
    r1.set("name", Value::Text("Alice".into()));
    r1.set("age",  Value::Integer(30));
    t.insert(r1).expect("insert Alice");

    let mut r2 = Row::new();
    r2.set("id",   Value::Integer(2));
    r2.set("name", Value::Text("Bob".into()));
    r2.set("age",  Value::Integer(25));
    t.insert(r2).expect("insert Bob");

    assert_eq!(t.count(), 2);
    println!("  inserted 2 rows  ✓");

    // UPDATE
    let updated = t.update_where(
        |r| r.get("name") == Some(&Value::Text("Alice".into())),
        |r| r.set("age", Value::Integer(31)),
    );
    assert_eq!(updated, 1);
    println!("  updated Alice's age  ✓");

    // DELETE
    let deleted = t.delete_where(|r| r.get("name") == Some(&Value::Text("Bob".into())));
    assert_eq!(deleted, 1);
    assert_eq!(t.count(), 1);
    println!("  deleted Bob  ✓");
}

// ── Demo 2: duplicate key rejection ──────────────────────────────────────────
fn demo_duplicate_key_rejection() {
    println!("[ Demo 2 ] duplicate key / missing PK");

    let mut db = Database::new();
    db.create_table("items", Schema::new("sku", vec!["sku", "price"]));
    let t = db.get_table_mut("items").unwrap();

    let mut r = Row::new();
    r.set("sku",   Value::Text("ABC".into()));
    r.set("price", Value::Float(9.99));
    t.insert(r.clone()).unwrap();

    // Duplicate key
    assert_eq!(t.insert(r), Err(DbError::DuplicateKey));
    println!("  duplicate key rejected  ✓");

    // Missing PK
    let mut bad = Row::new();
    bad.set("price", Value::Float(1.0));
    assert_eq!(t.insert(bad), Err(DbError::MissingPrimaryKey));
    println!("  missing PK rejected  ✓");
}

// ── Demo 3: filtered queries ──────────────────────────────────────────────────
fn demo_filtered_queries() {
    println!("[ Demo 3 ] select_where / delete_where");

    let mut db = Database::new();
    db.create_table("scores", Schema::new("id", vec!["id", "score"]));
    let t = db.get_table_mut("scores").unwrap();

    for i in 1..=5i64 {
        let mut r = Row::new();
        r.set("id",    Value::Integer(i));
        r.set("score", Value::Integer(i * 10));
        t.insert(r).unwrap();
    }

    let high = t.select_where(|r| {
        matches!(r.get("score"), Some(Value::Integer(s)) if *s >= 30)
    });
    assert_eq!(high.len(), 3, "scores 30, 40, 50 qualify");
    println!("  select_where returned {} rows  ✓", high.len());

    let removed = t.delete_where(|r| {
        matches!(r.get("score"), Some(Value::Integer(s)) if *s < 30)
    });
    assert_eq!(removed, 2);
    assert_eq!(t.count(), 3);
    println!("  delete_where removed {removed} rows  ✓");
}

// ── Demo 4: multiple tables in one database ───────────────────────────────────
fn demo_multi_table_database() {
    println!("[ Demo 4 ] multi-table database");

    let mut db = Database::new();
    db.create_table("customers", Schema::new("id",  vec!["id",  "name"]));
    db.create_table("orders",    Schema::new("oid", vec!["oid", "customer_id", "total"]));

    let c = db.get_table_mut("customers").unwrap();
    let mut cust = Row::new();
    cust.set("id",   Value::Integer(1));
    cust.set("name", Value::Text("Alice".into()));
    c.insert(cust).unwrap();

    let o = db.get_table_mut("orders").unwrap();
    let mut ord = Row::new();
    ord.set("oid",         Value::Integer(100));
    ord.set("customer_id", Value::Integer(1));
    ord.set("total",       Value::Float(49.99));
    o.insert(ord).unwrap();

    assert_eq!(db.get_table("customers").unwrap().count(), 1);
    assert_eq!(db.get_table("orders").unwrap().count(),    1);
    assert_eq!(db.table_count(), 2);
    println!("  2 independent tables confirmed  ✓");

    db.drop_table("orders");
    assert_eq!(db.table_count(), 1);
    println!("  drop_table works  ✓");
}
