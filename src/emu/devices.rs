use super::*;
pub trait Device {
    const MEMMAP: MemMapEntry;
}

#[derive(Debug, Clone)]
pub struct MemMapEntry {
    pub base: u64, pub size: u64,
}

// DRAM
pub struct DRAM(Vec<u8>);
impl Device for DRAM {
    const MEMMAP: MemMapEntry = MemMapEntry {
        base: DRAM_BASE,
        size: DRAM_SIZE as u64,
    };
}
