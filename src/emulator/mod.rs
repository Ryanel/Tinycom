pub mod cpu;
pub mod registers;

use isa;

pub struct Emulator<'a> {
    pub cpu: cpu::CPU<'a>,
    pub is_running : bool
}

impl<'a> Default for Emulator<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Emulator<'a> {
    pub fn new() -> Emulator<'a> {
        Emulator {
            cpu: cpu::CPU::new(),
            is_running: true
        }
    }

    pub fn run(&mut self) {   
        let inc = isa::Instruction::new(0x01CD_EFFF);
        let dec = isa::Instruction::new(0x02CD_EFFF);

        self.cpu.current_instruction = inc;
        let mut i = 3;
        while self.is_running 
        {
            self.cpu.current_instruction = dec;
            self.cpu.step();

            self.cpu.debug();

            i -= 1;
            if i == 0 {
                self.is_running = false;
            }
        }
    }
}