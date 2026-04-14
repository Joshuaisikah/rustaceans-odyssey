use super::address::{PhysAddr, VirtAddr};

// ─── PageEntry ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct PageEntry {
    pub frame_number: u64,
    pub present:      bool,
    pub writable:     bool,
    pub executable:   bool,
}

// ─── PageTable — flat (single-level) page table ───────────────────────────────
//
// Hint: HashMap<u64 (vpn), PageEntry>.

pub struct PageTable {
    // TODO: HashMap<u64, PageEntry>
}

impl PageTable {
    pub fn new() -> Self { todo!() }

    /// Map virtual page containing `virt` to physical frame at `phys`.
    pub fn map(&mut self, virt: VirtAddr, phys: PhysAddr, writable: bool, executable: bool) {
        todo!()
    }

    /// Unmap the page containing `virt`.  Returns true if a mapping existed.
    pub fn unmap(&mut self, virt: VirtAddr) -> bool { todo!() }

    /// Translate a virtual address → physical address (preserves offset).
    /// Returns None for unmapped pages (page fault).
    pub fn translate(&self, virt: VirtAddr) -> Option<PhysAddr> { todo!() }

    pub fn is_mapped(&self, virt: VirtAddr) -> bool { todo!() }

    pub fn get_entry(&self, virt: VirtAddr) -> Option<&PageEntry> { todo!() }
}

impl Default for PageTable {
    fn default() -> Self { Self::new() }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_preserves_offset() {
        let mut pt = PageTable::new();
        pt.map(VirtAddr(0x1000), PhysAddr(0x5000), true, false);
        assert_eq!(pt.translate(VirtAddr(0x1ABC)), Some(PhysAddr(0x5ABC)));
    }

    #[test]
    fn test_translate_unmapped_is_none() {
        let pt = PageTable::new();
        assert_eq!(pt.translate(VirtAddr(0xDEAD_0000)), None);
    }

    #[test]
    fn test_map_and_unmap() {
        let mut pt = PageTable::new();
        let v = VirtAddr(0x2000);
        pt.map(v, PhysAddr(0x9000), true, true);
        assert!(pt.is_mapped(v));
        assert!(pt.unmap(v));
        assert!(!pt.is_mapped(v));
    }

    #[test]
    fn test_unmap_not_mapped_returns_false() {
        let mut pt = PageTable::new();
        assert!(!pt.unmap(VirtAddr(0xF000_0000)));
    }

    #[test]
    fn test_remap_overwrites() {
        let mut pt = PageTable::new();
        let v = VirtAddr(0x3000);
        pt.map(v, PhysAddr(0x1000), true,  false);
        pt.map(v, PhysAddr(0x2000), false, false);
        assert_eq!(pt.translate(VirtAddr(0x3100)), Some(PhysAddr(0x2100)));
    }

    #[test]
    fn test_get_entry_flags() {
        let mut pt = PageTable::new();
        pt.map(VirtAddr(0x4000), PhysAddr(0x8000), true, false);
        let e = pt.get_entry(VirtAddr(0x4000)).unwrap();
        assert!(e.present && e.writable && !e.executable);
    }
}
