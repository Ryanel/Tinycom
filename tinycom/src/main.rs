// Import Crates
extern crate tinycom_common;
extern crate tinycom_emulator;

#[allow(unused_imports)]
#[macro_use]
extern crate log;
extern crate simplelog;

// Crate usings
use simplelog::*;

// Usings
use tinycom_common::isa::*;
use tinycom_emulator::cpu::*;

/// Main function
fn main() {
    // Setup logging
    let logging_enabled: bool = true;
    if logging_enabled {
        init_logging();
    }

    let mut cpu_main : CPU = CPU::new();
    cpu_main.current_instruction = Instruction::get(0x0A_00_00_00);

    cpu_main.debug_printstate();
}

/// Initialise logging
fn init_logging() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Trace, Config::default()).unwrap(),
    ]).unwrap();
}
