mod emu;

fn main() {
    let mut bus = emu::Bus::new(std::fs::read("code.bin").unwrap());
    let mut cpu = emu::CPU::new(&mut bus);

    loop {
        cpu.cycle().unwrap();
        cpu.dump_regs()
    }
}
