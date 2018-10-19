// Import Crates
#[macro_use]
extern crate log;
extern crate simplelog;

extern crate tinycom_common;

// Crate usings
use simplelog::*;

// Usings
use tinycom_common::cpu::isa::*;
use tinycom_common::cpu::*;

/// Main function
fn main() {
    // Setup logging
    let logging_enabled: bool = true;
    if logging_enabled {
        init_logging();
    }

    let mut cpu_main : CPU = CPU::new();
    cpu_main.current_instruction = Instruction::get(0x02_00_00_00);
    cpu_main.pc.add(0xFFFF_FFFF);
    warn!("Does PC overflow: {}", cpu_main.pc.add_checked(0xFFFF_FFFF));
    cpu_main.debug_printstate();
}

fn init_logging() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Trace, Config::default()).unwrap(),
    ]).unwrap();
}
