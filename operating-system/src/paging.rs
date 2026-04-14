// ─── Paging (flat single-level page table) ───────────────────────────────────

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtAddr(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysAddr(pub u64);

impl VirtAddr {
    /// Page number (virtual address / PAGE_SIZE).
    pub fn page_number(&self) -> u64 {
        self.0 / PAGE_SIZE as u64
    }

    /// Byte offset within the page.
    pub fn page_offset(&self) -> u64 {
        self.0 % PAGE_SIZE as u64
    }
}

impl PhysAddr {
    /// Physical frame number (physical address / PAGE_SIZE).
    pub fn frame_number(&self) -> u64 {
        self.0 / PAGE_SIZE as u64
    }
}

// ─── PageEntry ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct PageEntry {
    pub frame_number: u64,
    pub present:      bool,
    pub writable:     bool,
    pub executable:   bool,
}

// ─── PageTable ───────────────────────────────────────────────────────────────
//
// Flat single-level page table: VPN → PageEntry.
// Hint: HashMap<u64 (vpn), PageEntry>.

pub struct PageTable {
    // TODO: HashMap<u64, PageEntry>
}

impl PageTable {
    pub fn new() -> Self {
        todo!()
    }

    /// Map virtual page containing `virt` to the physical frame at `phys`.
    pub fn map(&mut self, virt: VirtAddr, phys: PhysAddr, writable: bool, executable: bool) {
        todo!()
    }

    /// Unmap the page containing `virt`.  Returns true if a mapping existed.
    pub fn unmap(&mut self, virt: VirtAddr) -> bool {
        todo!()
    }

    /// Translate a virtual address to a physical address.
    /// Preserves the page offset.  Returns None for unmapped pages (page fault).
    pub fn translate(&self, virt: VirtAddr) -> Option<PhysAddr> {
        todo!()
    }

    /// True if a present mapping exists for the page containing `virt`.
    pub fn is_mapped(&self, virt: VirtAddr) -> bool {
        todo!()
    }

    /// Return the PageEntry for the page containing `virt`, or None.
    pub fn get_entry(&self, virt: VirtAddr) -> Option<&PageEntry> {
        todo!()
    }
}

impl Default for PageTable {
    fn default() -> Self { Self::new() }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // page_number is address ÷ PAGE_SIZE.
    #[test]
    fn test_virt_addr_page_number() {
        assert_eq!(VirtAddr(8192).page_number(), 2);
    }

    // page_offset is address mod PAGE_SIZE.
    #[test]
    fn test_virt_addr_page_offset() {
        assert_eq!(VirtAddr(4096 + 256).page_offset(), 256);
    }

    // frame_number for a PhysAddr.
    #[test]
    fn test_phys_addr_frame_number() {
        assert_eq!(PhysAddr(PAGE_SIZE as u64 * 5).frame_number(), 5);
    }

    // translate returns the correct physical address (with offset).
    #[test]
    fn test_translate_mapped_address_preserves_offset() {
        let mut pt = PageTable::new();
        pt.map(VirtAddr(0x1000), PhysAddr(0x5000), true, false);
        assert_eq!(pt.translate(VirtAddr(0x1ABC)), Some(PhysAddr(0x5ABC)));
    }

    // translate returns None for an unmapped address.
    #[test]
    fn test_translate_unmapped_is_none() {
        let pt = PageTable::new();
        assert_eq!(pt.translate(VirtAddr(0xDEAD_0000)), None);
    }

    // is_mapped is true after map, false after unmap.
    #[test]
    fn test_map_and_unmap() {
        let mut pt = PageTable::new();
        let virt   = VirtAddr(0x2000);
        pt.map(virt, PhysAddr(0x9000), true, true);
        assert!(pt.is_mapped(virt));
        assert!(pt.unmap(virt));
        assert!(!pt.is_mapped(virt));
    }

    // unmap of a never-mapped address returns false.
    #[test]
    fn test_unmap_not_mapped_returns_false() {
        let mut pt = PageTable::new();
        assert!(!pt.unmap(VirtAddr(0xF000_0000)));
    }

    // Remapping the same virtual address overwrites the old mapping.
    #[test]
    fn test_remap_overwrites_old_mapping() {
        let mut pt = PageTable::new();
        let virt   = VirtAddr(0x3000);
        pt.map(virt, PhysAddr(0x1000), true,  false);
        pt.map(virt, PhysAddr(0x2000), false, false);
        assert_eq!(pt.translate(VirtAddr(0x3100)), Some(PhysAddr(0x2100)));
    }

    // get_entry returns the stored flags.
    #[test]
    fn test_get_entry_returns_correct_flags() {
        let mut pt = PageTable::new();
        pt.map(VirtAddr(0x4000), PhysAddr(0x8000), true, false);
        let entry = pt.get_entry(VirtAddr(0x4000)).unwrap();
        assert!(entry.present);
        assert!(entry.writable);
        assert!(!entry.executable);
    }

    // Multiple distinct pages can be mapped simultaneously.
    #[test]
    fn test_multiple_pages_mapped_independently() {
        let mut pt = PageTable::new();
        pt.map(VirtAddr(0x0000), PhysAddr(0xA000), true,  false);
        pt.map(VirtAddr(0x1000), PhysAddr(0xB000), false, true);
        assert_eq!(pt.translate(VirtAddr(0x0000)), Some(PhysAddr(0xA000)));
        assert_eq!(pt.translate(VirtAddr(0x1000)), Some(PhysAddr(0xB000)));
    }
}
