use super::*;

const KB: usize = 1024;
const MB: usize = KB * 1024;
pub const DRAM_SIZE: usize  = 64*MB;
pub const DRAM_BASE: u64    = 0x8000_0000;
pub const ROM_SIZE: usize   = 8*KB;
pub const ROM_BASE: u64     = 0x0000_0000;

pub struct Bus {
    pub devices: Vec<Box<dyn Device>>,
}

impl Bus {
    pub fn new(devices: Vec<Box<dyn Device>>) -> Self {
        Self {
            devices
        }
    }

    pub fn read(&self, addr: u64, size: u32) -> Result<u64> {
        for dev in self.devices.iter() {
            let dev_map = dev.get_memmap();
            let dev_range = dev_map.base..dev_map.base+dev_map.size;
            if dev_range.contains(&addr) {
                return dev.read(addr - dev_map.base, size);
            }
        }

        return Err(Box::new(Error::InvalidMemoryAccess));
    }

    pub fn write(&mut self, addr: u64, data: u64, size: u32) -> Result<()> {
        for (i, dev) in self.devices.iter_mut().enumerate() {
            let dev_map = dev.get_memmap();
            let dev_range = dev_map.base..dev_map.base+dev_map.size;
            if dev_range.contains(&addr) {
                return dev.write(addr - dev_map.base, data, size);
            }
        }

        return Err(Box::new(Error::InvalidMemoryAccess));
    }
}
