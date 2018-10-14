// Import Crates
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;
extern crate log;
extern crate simplelog;

// Define mods
pub mod isa;
pub mod emulator;

// Usings
use simplelog::*;
use emulator::*;

/// Main function
fn main() {
    // Setup logging
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Trace, Config::default()).unwrap(),
    ]).unwrap();

    let mut emu = Emulator::new();
    emu.run();
}
