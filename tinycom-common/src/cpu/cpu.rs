use cpu::isa::*;
use cpu::*;

pub struct CPU {
    pub current_instruction: Instruction,
    pub pc : Register<u32>,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> CPU {
        debug!("Creating CPU");
        CPU {
            current_instruction: Instruction::get(0x0000_0000), // NOP
            pc: Register {value: 0}
        }
    }

    pub fn debug_printstate(self : Self) {
        debug!("Current Instruction: {:?}", self.current_instruction);
        debug!("PC: {:?}", self.pc);
    }
}