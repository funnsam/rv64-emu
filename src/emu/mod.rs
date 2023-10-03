pub mod cpu;
pub use cpu::*;
pub mod bus;
pub use bus::*;
pub mod devices;
pub use devices::*;
mod insts;
use insts::*;

pub type Result<A> = std::result::Result<A, Box<dyn std::error::Error>>;
