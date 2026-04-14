// ─── memory — virtual memory subsystem ───────────────────────────────────────

pub mod address;
pub mod page_table;

pub use address::{PhysAddr, VirtAddr, PAGE_SIZE};
pub use page_table::{PageEntry, PageTable};
