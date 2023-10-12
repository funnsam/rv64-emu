use super::*;
pub trait Device {
    fn get_memmap(&self) -> MemMapEntry;

    fn read(&self, addr: u64, size: u32) -> Result<u64>;
    fn write(&mut self, addr: u64, data: u64, size: u32) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct MemMapEntry {
    pub base: u64, pub size: u64,
}

// DRAM
pub struct DRAM(Vec<u8>);
impl Device for DRAM {
    fn get_memmap(&self) -> MemMapEntry {
        MemMapEntry {
            base: DRAM_BASE,
            size: DRAM_SIZE as u64,
        }
    }

    fn read(&self, addr: u64, size: u32) -> Result<u64> {
        match size {
            B => Ok(*self.0.get(addr as usize).ok_or(Error::InvalidMemoryAccess)? as u64),
            H => Ok(u16::from_le_bytes(self.0[addr as usize..addr as usize + 2].try_into().unwrap()) as u64),
            W => Ok(u32::from_le_bytes(self.0[addr as usize..addr as usize + 4].try_into().unwrap()) as u64),
            D => Ok(u64::from_le_bytes(self.0[addr as usize..addr as usize + 8].try_into().unwrap()) as u64),
            _ => Err(Box::new(Error::InvalidMemoryAccess))
        }
    }
    
    fn write(&mut self, addr: u64, data: u64, size: u32) -> Result<()> {
        match size {
            B => *self.0.get_mut(addr as usize).ok_or(Error::InvalidMemoryAccess)? = data as u8,
            H => {
                *self.0.get_mut(addr as usize    ).ok_or(Error::InvalidMemoryAccess)? = (data & 0xff) as u8;
                *self.0.get_mut(addr as usize + 1).ok_or(Error::InvalidMemoryAccess)? = ((data >> 8) & 0xff) as u8;
            },
            W => {
                *self.0.get_mut(addr as usize    ).ok_or(Error::InvalidMemoryAccess)? = (data & 0xff) as u8;
                *self.0.get_mut(addr as usize + 1).ok_or(Error::InvalidMemoryAccess)? = ((data >> 8) & 0xff) as u8;
                *self.0.get_mut(addr as usize + 2).ok_or(Error::InvalidMemoryAccess)? = ((data >> 16) & 0xff) as u8;
                *self.0.get_mut(addr as usize + 3).ok_or(Error::InvalidMemoryAccess)? = ((data >> 24) & 0xff) as u8;
            },
            D => {
                *self.0.get_mut(addr as usize    ).ok_or(Error::InvalidMemoryAccess)? = (data & 0xff) as u8;
                *self.0.get_mut(addr as usize + 1).ok_or(Error::InvalidMemoryAccess)? = ((data >> 8) & 0xff) as u8;
                *self.0.get_mut(addr as usize + 2).ok_or(Error::InvalidMemoryAccess)? = ((data >> 16) & 0xff) as u8;
                *self.0.get_mut(addr as usize + 3).ok_or(Error::InvalidMemoryAccess)? = ((data >> 24) & 0xff) as u8;
                *self.0.get_mut(addr as usize + 4).ok_or(Error::InvalidMemoryAccess)? = ((data >> 32) & 0xff) as u8;
                *self.0.get_mut(addr as usize + 5).ok_or(Error::InvalidMemoryAccess)? = ((data >> 40) & 0xff) as u8;
                *self.0.get_mut(addr as usize + 6).ok_or(Error::InvalidMemoryAccess)? = ((data >> 48) & 0xff) as u8;
                *self.0.get_mut(addr as usize + 7).ok_or(Error::InvalidMemoryAccess)? = ((data >> 56) & 0xff) as u8;
            },
            _ => return Err(Box::new(Error::InvalidMemoryAccess))
        }

        Ok(())
    }
}

impl DRAM {
    pub fn new(code: Vec<u8>) -> Self {
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram.splice(..code.len(), code.iter().cloned());

        Self(dram)
    }
}

// ROM
pub struct ROM(Vec<u8>);
impl Device for ROM {
    fn get_memmap(&self) -> MemMapEntry {
        MemMapEntry {
            base: ROM_BASE,
            size: ROM_SIZE as u64,
        }
    }

    fn read(&self, addr: u64, size: u32) -> Result<u64> {
        match size {
            B => Ok(*self.0.get(addr as usize).ok_or(Error::InvalidMemoryAccess)? as u64),
            H => Ok(u16::from_le_bytes(self.0[addr as usize..addr as usize + 2].try_into().unwrap()) as u64),
            W => Ok(u32::from_le_bytes(self.0[addr as usize..addr as usize + 4].try_into().unwrap()) as u64),
            D => Ok(u64::from_le_bytes(self.0[addr as usize..addr as usize + 8].try_into().unwrap()) as u64),
            _ => Err(Box::new(Error::InvalidMemoryAccess))
        }
    }
    
    fn write(&mut self, addr: u64, data: u64, size: u32) -> Result<()> {
        Err(Box::new(Error::InvalidMemoryAccess))
    }
}

impl ROM {
    pub fn new(code: Vec<u8>) -> Self {
        let mut rom = vec![0; ROM_SIZE as usize];
        rom.splice(..code.len(), code.iter().cloned());

        Self(rom)
    }
}

// Debug port
pub struct DebugPort;
impl Device for DebugPort {
    fn get_memmap(&self) -> MemMapEntry {
        MemMapEntry {
            base: 0x1_0000_0000,
            size: 8,
        }
    }

    fn read(&self, addr: u64, size: u32) -> Result<u64> {
        Err(Box::new(Error::InvalidMemoryAccess))
    }

    fn write(&mut self, addr: u64, data: u64, size: u32) -> Result<()> {
        println!("{data}");
        Ok(())
    }
}
