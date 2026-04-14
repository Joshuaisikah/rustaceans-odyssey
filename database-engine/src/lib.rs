// ─── database-engine ─────────────────────────────────────────────────────────
//
// A minimal in-memory relational database.
//
// Crate layout
// ┌─────────────────────────────────────────────────────────┐
// │ types/          Core domain types                       │
// │   value.rs   →  Value enum                             │
// │   row.rs     →  Row  (column → Value map)              │
// │   schema.rs  →  Schema (column list + primary key)     │
// ├─────────────────────────────────────────────────────────┤
// │ storage/        Persistence layer                       │
// │   table.rs   →  Table (insert / select / update / del) │
// ├─────────────────────────────────────────────────────────┤
// │ error.rs        DbError enum                           │
// │ database.rs     Database (collection of named Tables)  │
// └─────────────────────────────────────────────────────────┘

pub mod database;
pub mod error;
pub mod storage;
pub mod types;

// Flatten the public API so callers can write `use database_engine::Row`
// rather than `use database_engine::types::Row`.
pub use database::Database;
pub use error::DbError;
pub use storage::Table;
pub use types::{Row, Schema, Value};
