use std::fmt;
use num_traits::{FromPrimitive};

// Opcodes
// -------------------------------------------------------------

/// An opcode is a single operation that can be performed on the CPU
#[derive(Copy, Clone, Debug, Primitive)]
pub enum Opcode {
    NOP = 0x00,
    INC = 0x01,
    DEC = 0x02,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Opcode::NOP => String::from("NOP"),
            Opcode::INC => String::from("INC A"),
            Opcode::DEC => String::from("DEC A"),
            //_           => String::from("???")
        };
        write!(f, "{}", s)
    }
}

impl Opcode {
    fn symbol(op: u8) -> Opcode {
        let element = Opcode::from_u8(op);
        match element {
            Some(op) => op,
            None => {
                println!("Unknown opcode!");
                Opcode::NOP
            }
        }
    }
}

// Instructions
// -------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
/// A decoded CPU instruction
pub struct Instruction {
    /// All of the bytes that make up the instruction. All instructions are 4 bytes long
    pub bytes: u32,
    /// The opcode for this instruction. There are 256 opcodes, so it fits in a u8
    pub opcode: Opcode,
}

impl Instruction {
    /// Returns a new instruction from a u32, with information pre-filled out
    pub fn new(bytes: u32) -> Instruction {
        let code: u8 = ((bytes & 0xFF00_0000) >> 24) as u8;
        Instruction {
            bytes,
            opcode: Opcode::symbol(code),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "0x{:02X} ({}) : (0x{:08X})",
            self.opcode as u8, self.opcode, self.bytes
        )
    }
}
