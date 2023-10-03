use super::*;

#[derive(Debug)]
pub struct CPU<'a> {
    reg: [u64; 31],
    pc : u64,
    bus: &'a mut Bus
}

impl<'a> CPU<'a> {
    pub fn new(bus: &'a mut Bus) -> Self {
        let mut reg = [0; 31];
        reg[1] = DRAM_SIZE as u64 + DRAM_BASE; // x2 (sp)
        Self {
            reg,
            pc : DRAM_BASE,
            bus
        }
    }

    #[inline]
    pub fn read_reg(&self, nth: usize) -> u64 {
        *self.reg.get(nth-1).unwrap_or(&0)
    }

    #[inline]
    pub fn write_reg(&mut self, nth: usize, d: u64) {
        if nth != 0 {
            self.reg[nth-1] = d;
        }
    }

    const ABI_REGS: [&str; 31] = [
        "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0",
        "a1", "a2", "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5",
        "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6",
    ];

    pub fn dump_regs(&self) {
        println!("pc:\t{:016x}", self.pc);
        for i in 1..32 {
            if self.reg[i-1] != 0 {
                println!("{}:\t{:016x}", Self::ABI_REGS[i-1], self.reg[i-1]);
            } else if *self.reg.get(i-2).unwrap_or(&1) != 0 {
                println!("...");
            }
        }
        println!()
    }

    #[must_use]
    pub fn cycle(&mut self) -> Result<()> {
        let i = self.fetch()?;
        println!("{i:08x}");
        self.pc += 4;
        self.execute(i)
    }

    pub fn fetch(&self) -> Result<u32> {
        self.bus.read(self.pc, W).map(|a| a as u32)
    }

    pub fn execute(&mut self, inst: u32) -> Result<()> {
        let opcode = inst & 0x7f;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;

        let rs2 = ((inst >> 20) & 0x1f) as usize;
        let funct3 = (inst >> 12) & 0x7;
        let funct7 = (inst >> 25) & 0x7f;

        let u_imm_s = (inst as i32 as i64 >> 12) as u64;
        let i_imm_s = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
        let s_imm_s = (((inst & 0xfe000000) as i32 as i64 >> 20) as u64) | ((inst >> 7) & 0x1f) as u64;
        let j_imm_s = (((inst & 0x80000000) as i32 as i64 >> 11) as u64) | (inst & 0xff000) as u64 | ((inst >> 9) & 0x800) as u64 | ((inst >> 20) & 0x7fe) as u64;
        let b_imm_s = (((inst & 0x80000000) as i32 as i64 >> 19) as u64) | ((inst & 0x80) << 4) as u64 | ((inst >> 20) & 0x7e0) as u64 | ((inst >> 7) & 0x1e) as u64;

        match opcode {
            LUI => {
                self.write_reg(rd, u_imm_s << 12);
            },
            AUIPC => {
                self.write_reg(rd, self.pc + (u_imm_s << 12));
            },
            JAL => {
                self.write_reg(rd, self.pc);
                self.pc += j_imm_s - 4;
            },
            JALR => {
                let pc = self.pc;
                self.pc = (self.read_reg(rs1) + i_imm_s) & !1;
                self.write_reg(rd, pc);
            },
            BCOND => {
                let a = self.read_reg(rs1);
                let b = self.read_reg(rs2);
                if match funct3 {
                    0x0 => a == b,
                    0x1 => a != b,
                    0x4 => (a as i64) <  b as i64,
                    0x5 => (a as i64) >= b as i64,
                    0x6 => a <  b,
                    0x7 => a >= b,
                    _ => Err(Error::UnsupportedInst(inst))?,
                } {
                    self.pc += b_imm_s - 4;
                }
            },
            ALU => {
                let a = self.read_reg(rs1);
                let b = self.read_reg(rs2);
                let shamt = ((self.read_reg(rs2) & 0x3f) as u64) as u32;
                self.write_reg(rd, match (funct3, funct7) {
                    (ADD , 0x00) => a + b,
                    (SLT , 0x00) => ((a as i64) < b as i64) as u64,
                    (SLTU, 0x00) => (a < b) as u64,
                    (XOR , 0x00) => a ^ b,
                    (OR  , 0x00) => a | b,
                    (AND , 0x00) => a & b,
                    (SLL , 0x00) => a << shamt,
                    (SRX , 0x00) => a >> shamt,
                    (SRX , 0x20) => (a as i64 >> shamt) as u64,
                    SUB    => a - b,
                    MUL    => a * b,
                    MULH   => ((a as u128 * b as u128) >> 64) as u64,
                    MULHU  => ((a as i64 as i128 * b as i64 as i128) >> 64) as u64,
                    MULHSU => ((a as i64 as i128 * b as i128) >> 64) as u64,
                    DIV    => (a as i64 / b as i64) as u64,
                    DIVU   => a / b,
                    REM    => (a as i64 % b as i64) as u64,
                    REMU   => a % b,
                    _ => Err(Error::UnsupportedInst(inst))?,
                })
            },
            ALUI => {
                let a = self.read_reg(rs1);
                let b = i_imm_s;
                let shamt = (b & 0x3f) as u32;
                self.write_reg(rd, match (funct3, funct7) {
                    (ADD , _) => a + b,
                    (SLT , _) => ((a as i64) < b as i64) as u64,
                    (SLTU, _) => (a < b) as u64,
                    (XOR , _) => a ^ b,
                    (OR  , _) => a | b,
                    (AND , _) => a & b,
                    (SLL , 0x00) => a << shamt,
                    (SRX , 0x00) => a >> shamt,
                    (SRX , 0x20) => (a as i64 >> shamt) as u64,
                    _ => Err(Error::UnsupportedInst(inst))?,
                })
            },
            ALUW => {
                let a = self.read_reg(rs1);
                let b = self.read_reg(rs2);
                let shamt = (b & 0x3f) as u32;
                self.write_reg(rd, match (funct3, funct7) {
                    (ADD, 0x00) => (a as i32 + b as i32) as i32 as i64 as u64,
                    (SLL, 0x00) => ((a as u32) << shamt) as i32 as i64 as u64,
                    (SRX, 0x00) => ((a as u32) >> shamt) as i32 as i64 as u64,
                    (SRX, 0x20) => ((a as i32) >> (shamt as i32)) as u64,

                    SUB  => (a as i32 - b as i32) as i32 as i64 as u64,
                    MUL  => (a as u32 * b as u32) as i32 as i64 as u64,
                    DIV  => (a as i32 / b as i32) as i32 as i64 as u64,
                    DIVU => (a as u32 / b as u32) as i32 as i64 as u64,
                    REM  => (a as i32 % b as i32) as i32 as i64 as u64,
                    REMU => (a as u32 % b as u32) as i32 as i64 as u64,
                    _ => Err(Error::UnsupportedInst(inst))?,
                })
            },
            ALUIW => {
                let a = self.read_reg(rs1);
                let b = i_imm_s;
                let shamt = ((self.read_reg(rs2) & 0x3f) as u64) as u32;
                self.write_reg(rd, match (funct3, funct7) {
                    (ADD, _) => (a as i32 + b as i32) as i32 as i64 as u64,
                    (SLL, 0x00) => ((a as u32) << shamt) as i32 as i64 as u64,
                    (SRX, 0x00) => ((a as u32) >> shamt) as i32 as i64 as u64,
                    (SRX, 0x20) => ((a as i32) >> (shamt as i32)) as u64,
                    _ => Err(Error::UnsupportedInst(inst))?,
                })
            },
            LOAD => {
                let addr = self.read_reg(rs1) + i_imm_s;
                let val = self.bus.read(addr, funct3 & 3)?;
                self.write_reg(rd, match funct3 {
                    B => val as i8 as i64 as u64,
                    H => val as i16 as i64 as u64,
                    W => val as i32 as i64 as u64,
                    D | BU | HU | WU => val,
                    _ => unreachable!()
                })
            },
            STOR => {
                let addr = self.read_reg(rs1) + s_imm_s;
                self.bus.write(addr, self.read_reg(rs2), funct3 & 3)?;
            },
            AMO => {
                let rl = (funct7 & 1) != 0;
                let aq = ((funct7 >> 1) & 1) != 0;
                match funct7 >> 2 {
                    _ => Err(Error::UnsupportedInst(inst))?
                }
            },
            _ => Err(Error::UnsupportedInst(inst))?
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    UnsupportedInst(u32),
    InvalidMemoryAccess,
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
