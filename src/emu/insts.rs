pub const LUI  : u32 = 0x37;
pub const AUIPC: u32 = 0x17;
pub const JAL  : u32 = 0x6F;
pub const JALR : u32 = 0x67;
pub const BCOND: u32 = 0x63;
pub const LOAD : u32 = 0x03;
pub const STOR : u32 = 0x23;
pub const ALUI : u32 = 0x13;
pub const ALU  : u32 = 0x33;
pub const ALUIW: u32 = 0x1B;
pub const ALUW : u32 = 0x3B;

// load and store
pub const B : u32 = 0x0;
pub const H : u32 = 0x1;
pub const W : u32 = 0x2;
pub const D : u32 = 0x3;
pub const BU: u32 = 0x4;
pub const HU: u32 = 0x5;
pub const WU: u32 = 0x6;

// alu
pub const ADD : u32 = 0x0;
pub const SLT : u32 = 0x2;
pub const SLTU: u32 = 0x3;
pub const XOR : u32 = 0x4;
pub const OR  : u32 = 0x6;
pub const AND : u32 = 0x7;
pub const SLL : u32 = 0x1;
pub const SRX : u32 = 0x5;
