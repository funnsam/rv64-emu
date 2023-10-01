pub mod cpu;
pub use cpu::*;
pub mod bus;
pub use bus::*;
mod insts;
use insts::*;

pub type Result<A> = std::result::Result<A, Box<dyn std::error::Error>>;
