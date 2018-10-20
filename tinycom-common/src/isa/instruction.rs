use isa::Opcode;

#[derive(Debug)]
/// A CPU instruction
pub struct Instruction {
    /// The full dword for this instruction.
    pub dword: u32,
    /// The opcode, pulled from the
    pub opcode: Opcode
}

impl Instruction {
    /// Returns a new instruction from a u32, with information pre-filled out
    pub fn get(data: u32) -> Instruction {
        let code: u8 = ((data & 0xFF00_0000) >> 24) as u8;
        Instruction {
            dword: data,
            opcode: Opcode::to_enum(code),
        }
    }
}