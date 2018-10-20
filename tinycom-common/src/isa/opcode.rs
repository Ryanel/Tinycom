use std::fmt;
use num_traits::{FromPrimitive};

#[derive(Copy, Clone, Debug, Primitive)]
pub enum Opcode {
    NOP = 0x00,
    INC = 0x01,
    DEC = 0x02,
}

impl Opcode {
    pub fn to_enum(op: u8) -> Opcode {
        let panic_unknown_opcode : bool = true;
        
        let element = Opcode::from_u8(op);

        match element {
            Some(op) => op,
            None => {
                error!("Attempted to decode unknown opcode (0x{:02X})!", op);
                if panic_unknown_opcode {
                    panic!("Unknown opcode");
                }
                else {
                    Opcode::NOP
                }
            }
        }
    }
}

#[allow(unreachable_patterns)]
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Opcode::NOP => String::from("NOP"),
            Opcode::INC => String::from("INC A"),
            Opcode::DEC => String::from("DEC A"),
            _           => String::from("???")
        };
        write!(f, "{}", s)
    }
}
