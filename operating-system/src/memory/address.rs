// ─── Virtual and Physical address types ──────────────────────────────────────

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtAddr(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysAddr(pub u64);

impl VirtAddr {
    /// Page number: virtual address / PAGE_SIZE.
    pub fn page_number(&self) -> u64 { self.0 / PAGE_SIZE as u64 }

    /// Byte offset within the page.
    pub fn page_offset(&self) -> u64 { self.0 % PAGE_SIZE as u64 }
}

impl PhysAddr {
    /// Physical frame number: physical address / PAGE_SIZE.
    pub fn frame_number(&self) -> u64 { self.0 / PAGE_SIZE as u64 }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_page_number()  { assert_eq!(VirtAddr(8192).page_number(), 2); }
    #[test] fn test_page_offset()  { assert_eq!(VirtAddr(4096 + 256).page_offset(), 256); }
    #[test] fn test_frame_number() { assert_eq!(PhysAddr(PAGE_SIZE as u64 * 5).frame_number(), 5); }

    #[test]
    fn test_address_arithmetic_consistency() {
        let v = VirtAddr(12345);
        let pg = v.page_number();
        let of = v.page_offset();
        assert_eq!(pg * PAGE_SIZE as u64 + of, v.0);
    }

    #[test]
    fn test_page_aligned_address_has_zero_offset() {
        let v = VirtAddr(4096 * 7);
        assert_eq!(v.page_offset(), 0);
    }
}
