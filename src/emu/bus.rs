use super::*;

const KB: usize = 1024;
const MB: usize = KB * 1024;
pub const DRAM_SIZE: usize = 64*MB;
pub const DRAM_BASE: u64 = 0x8000_0000;

#[derive(Debug)]
pub struct Bus {
    pub dram: Vec<u8>
}

impl Bus {
    pub fn new(code: Vec<u8>) -> Self {
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram.splice(..code.len(), code.iter().cloned());

        Self {
            dram
        }
    }

    pub fn read(&self, addr: u64, size: u32) -> Result<u64> {
        if DRAM_BASE <= addr {
            Ok(match size {
                B => self.dram_read_8 (addr - DRAM_BASE)? as u64,
                H => self.dram_read_16(addr - DRAM_BASE)? as u64,
                W => self.dram_read_32(addr - DRAM_BASE)? as u64,
                D => self.dram_read_64(addr - DRAM_BASE)? as u64,
                _ => unreachable!()
            })
        } else {
            Err(Box::new(Error::InvalidMemoryAccess))
        }
    }

    fn dram_read_8(&self, addr: u64) -> Result<u8> {
        Ok(*self.dram.get(addr as usize).ok_or(Error::InvalidMemoryAccess)?)
    }

    fn dram_read_16(&self, addr: u64) -> Result<u16> {
        Ok(u16::from_le_bytes(self.dram[addr as usize..addr as usize + 2].try_into().unwrap()))
    }

    fn dram_read_32(&self, addr: u64) -> Result<u32> {
        Ok(u32::from_le_bytes(self.dram[addr as usize..addr as usize + 4].try_into().unwrap()))
    }

    fn dram_read_64(&self, addr: u64) -> Result<u64> {
        Ok(u64::from_le_bytes(self.dram[addr as usize..addr as usize + 8].try_into().unwrap()))
    }

    pub fn write(&mut self, addr: u64, data: u64, size: u32) -> Result<()> {
        if DRAM_BASE <= addr {
            match size {
                B => self.dram_write_8 (addr - DRAM_BASE, data as u8) ,
                H => self.dram_write_16(addr - DRAM_BASE, data as u16),
                W => self.dram_write_32(addr - DRAM_BASE, data as u32),
                D => self.dram_write_64(addr - DRAM_BASE, data as u64),
                _ => unreachable!()
            }?;

            Ok(())
        } else {
            Err(Box::new(Error::InvalidMemoryAccess))
        }
    }

    fn dram_write_8(&mut self, addr: u64, data: u8) -> Result<()> {
        *self.dram.get_mut(addr as usize).ok_or(Error::InvalidMemoryAccess)? = data;
        Ok(())
    }

    fn dram_write_16(&mut self, addr: u64, data: u16) -> Result<()> {
        *self.dram.get_mut(addr as usize    ).ok_or(Error::InvalidMemoryAccess)? = (data & 0xff) as u8;
        *self.dram.get_mut(addr as usize + 1).ok_or(Error::InvalidMemoryAccess)? = ((data >> 8) & 0xff) as u8;
        Ok(())
    }

    fn dram_write_32(&mut self, addr: u64, data: u32) -> Result<()> {
        *self.dram.get_mut(addr as usize    ).ok_or(Error::InvalidMemoryAccess)? = (data & 0xff) as u8;
        *self.dram.get_mut(addr as usize + 1).ok_or(Error::InvalidMemoryAccess)? = ((data >> 8) & 0xff) as u8;
        *self.dram.get_mut(addr as usize + 2).ok_or(Error::InvalidMemoryAccess)? = ((data >> 16) & 0xff) as u8;
        *self.dram.get_mut(addr as usize + 3).ok_or(Error::InvalidMemoryAccess)? = ((data >> 24) & 0xff) as u8;
        Ok(())
    }

    fn dram_write_64(&mut self, addr: u64, data: u64) -> Result<()> {
        *self.dram.get_mut(addr as usize    ).ok_or(Error::InvalidMemoryAccess)? = (data & 0xff) as u8;
        *self.dram.get_mut(addr as usize + 1).ok_or(Error::InvalidMemoryAccess)? = ((data >> 8) & 0xff) as u8;
        *self.dram.get_mut(addr as usize + 2).ok_or(Error::InvalidMemoryAccess)? = ((data >> 16) & 0xff) as u8;
        *self.dram.get_mut(addr as usize + 3).ok_or(Error::InvalidMemoryAccess)? = ((data >> 24) & 0xff) as u8;
        *self.dram.get_mut(addr as usize + 4).ok_or(Error::InvalidMemoryAccess)? = ((data >> 32) & 0xff) as u8;
        *self.dram.get_mut(addr as usize + 5).ok_or(Error::InvalidMemoryAccess)? = ((data >> 40) & 0xff) as u8;
        *self.dram.get_mut(addr as usize + 6).ok_or(Error::InvalidMemoryAccess)? = ((data >> 48) & 0xff) as u8;
        *self.dram.get_mut(addr as usize + 7).ok_or(Error::InvalidMemoryAccess)? = ((data >> 56) & 0xff) as u8;
        Ok(())
    }
}
