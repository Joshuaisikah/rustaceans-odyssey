// ─── async-runtime ───────────────────────────────────────────────────────────
//
// A minimal single-threaded async executor built from scratch.
//
// Module layout:
//   future   — Ready<T> and YieldNow primitives
//   executor — cooperative task executor

pub mod executor;
pub mod future;

pub use executor::Executor;
pub use future::{ready, yield_now, Ready, YieldNow};
