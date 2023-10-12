mod emu;
use emu::*;

fn main() {
    let dram = DRAM::new(std::fs::read("code.bin").unwrap());
    let mut bus = emu::Bus::new(vec![Box::new(dram), Box::new(DebugPort)]);
    let mut cpu = emu::Core::new(&mut bus, 0);

    loop {
        cpu.cycle().unwrap();
        // cpu.dump_regs();
        // std::thread::sleep_ms(100);
    }
}
