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

pub const SUB   : (u32, u32) = (0x0, 0x20);
pub const MUL   : (u32, u32) = (0x0, 0x01);
pub const MULH  : (u32, u32) = (0x1, 0x01);
pub const MULHU : (u32, u32) = (0x3, 0x01);
pub const MULHSU: (u32, u32) = (0x2, 0x01);
pub const DIV   : (u32, u32) = (0x4, 0x01);
pub const DIVU  : (u32, u32) = (0x5, 0x01);
pub const REM   : (u32, u32) = (0x6, 0x01);
pub const REMU  : (u32, u32) = (0x7, 0x01);
