// Import Crates
#[macro_use]
extern crate enum_primitive_derive;
#[macro_use]
extern crate log;
extern crate num_traits;
extern crate simplelog;

// Crate usings
use simplelog::*;

// Mods
pub mod cpu;

// Usings
use cpu::isa::*;

/// Main function
fn main() {
    // Setup logging
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Trace, Config::default()).unwrap(),
    ]).unwrap();
    let x = Instruction::get(0x02_00_00_00);
    info!("Instruction:\n{:#?}", x);
}
