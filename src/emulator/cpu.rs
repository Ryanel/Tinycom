use isa::*;
use log::*;
use emulator::registers::*;

/// The CPU
pub struct CPU<'a> {
    /// The current instruction
    pub current_instruction: Instruction,
    /// The Program Counter
    pub pc: Register<'a>,
    /// Accumulator
    pub a: Register<'a>,
    // Stats
    cycle_budget: u64,

    pub is_running : bool
}

impl<'a> Default for CPU<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CPU<'a> {
    pub fn new() -> CPU<'a> {
        debug!("Creating CPU");

        CPU {
            current_instruction: Instruction::new(0),
            cycle_budget: 0,
            pc: Register::new("PC", RegisterSize::Bits32),
            a: Register::new("A", RegisterSize::Bits16),
            is_running: true
        }
    }

    /// Print the current contents of the registers
    pub fn debug_print_registers(&self) {
        debug!("{}", self.pc);
        debug!("{}", self.a);
    }

    /// Print debug info
    pub fn debug(&self) {
        debug!("Cycle: {}", self.cycle_budget);
        self.debug_print_registers();
    }

    /// Step one cycle in time
    pub fn step(&mut self) {
        let to_execute: Instruction = self.current_instruction;
        
        let result = self.execute(to_execute);

        self.cycle_budget += result.0;
        self.pc.add(result.1 * 4);
    }

    /// Execute an instruction
    pub fn execute(&mut self, ins: Instruction) -> (u64,u64) {
        trace!("Executing {}", ins);

        match ins.opcode {
            Opcode::NOP => {
                (4,1)
            }
            Opcode::INC => {
                self.a.add(1);
                (4,1)
            }
            Opcode::DEC => {
                self.a.sub(1);
                (4,1)
            }
            /*
            _ => {
                (4,1)
            }
            */
        }
    }
}

