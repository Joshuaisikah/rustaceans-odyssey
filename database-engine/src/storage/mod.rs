// ─── storage — persistence layer ─────────────────────────────────────────────
//
// Owns Table: the in-memory storage engine for a single relation.

pub mod table;

pub use table::Table;
