// Module: cpu

pub mod isa;

pub mod cpu;
pub use self::cpu::CPU;

pub mod register;
pub use self::register::Register;