// ─── types — core domain types ────────────────────────────────────────────────
//
// Everything the rest of the crate operates on: values, rows, schemas.

pub mod row;
pub mod schema;
pub mod value;

pub use row::Row;
pub use schema::Schema;
pub use value::Value;
