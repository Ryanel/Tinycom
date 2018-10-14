//-----------------------------------------------------------------------------
// registers.rs
// Define and implement a CPU register
//-----------------------------------------------------------------------------

use std::fmt;

#[derive(Copy, Clone)]
/// The size of a register
pub enum RegisterSize {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

/// A CPU Register
pub struct Register<'a> {
    /// The display name of the register
    pub name: &'a str,
    /// The value of this register.
    pub val: u64,
    /// The size of this register (bit width)
    pub size: RegisterSize,
}

impl<'a> Register<'a> {
    /// Returns a new instruction from a u32, with information pre-filled out
    pub fn new(name: &str, size: RegisterSize) -> Register {
        Register {
            val: 0,
            name,
            size,
        }
    }

    /// Change the value of the register, with appropreate wrapping
    pub fn set(&mut self, new_value: u64) -> u64 {
        self.val = new_value;

        match self.size {
            RegisterSize::Bits8 => {
                self.val &= 0xFF;
            }
            RegisterSize::Bits16 => {
                self.val &= 0xFFFF;
            }
            RegisterSize::Bits32 => {
                self.val &= 0xFFFF_FFFF;
            }
            _ => {
                panic!("Unimplemented");
            }
        }
        self.val
    }

    pub fn add(&mut self, val: u64) -> u64 {
        let new_val = self.val.wrapping_add(val);
        self.set(new_val);
        self.val
    }

    pub fn sub(&mut self, val: u64) -> u64 {
        let new_val = self.val.wrapping_sub(val);
        self.set(new_val);
        self.val
    }

    pub fn u8(&self) -> u8 {
        ((self.val & 0xFF) as u8)
    }

    pub fn u16(&self) -> u16 {
        ((self.val & 0xFFFF) as u16)
    }

    pub fn u32(&self) -> u32 {
        ((self.val & 0xFFFF_FFFF) as u32)
    }

    pub fn u64(&self) -> u64 {
        self.val
    }
}

impl<'a> fmt::Display for Register<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.size {
            RegisterSize::Bits8 => write!(f, "[{:>2} : 0x{:02X}]", self.name, self.val),
            RegisterSize::Bits16 => write!(f, "[{:>2} : 0x{:04X}]", self.name, self.val),
            RegisterSize::Bits32 => write!(f, "[{:>2} : 0x{:08X}]", self.name, self.val),
            RegisterSize::Bits64 => write!(f, "[{:>2} : 0x{:016X}]", self.name, self.val),
        }
    }
}

//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn register_add_basic() {
        let mut reg: Register = Register::new("A", RegisterSize::Bits8);
        reg.set(10);
        reg.add(5);
        assert_eq!(reg.val, 15);
    }

    #[test]
    fn register_demo() {
        let mut demoreg: Register = Register::new("D", RegisterSize::Bits8);
        //let mut a: Register = Register::new("A", RegisterSize::Bits8);
        let mut pc: Register = Register::new("PC", RegisterSize::Bits16);
        {
            let mut val = demoreg.set(5);
            val = demoreg.add(val);
            assert_eq!(val, 10);
        }
        {
            demoreg.set(100);
            let val = demoreg.sub(101);
            assert_eq!(val, 255);
        }
        {
            pc.set(6401);
            let pc_u8 = pc.u8();
            pc.set(pc_u8.into());
            assert_eq!(pc.val, 1);
        }
    }

    #[test]
    fn register_sub_basic() {
        let mut reg: Register = Register::new("A", RegisterSize::Bits8);
        reg.set(10);
        reg.sub(5);
        assert_eq!(reg.val, 5);
    }

    #[test]
    fn register_add_8bitwrap() {
        let mut reg: Register = Register::new("A", RegisterSize::Bits8);
        reg.set(255);
        reg.add(1);
        assert_eq!(reg.val, 0);
    }

    #[test]
    fn register_sub_8bitwrap() {
        let mut reg: Register = Register::new("A", RegisterSize::Bits8);
        reg.set(10);
        reg.sub(11);
        assert_eq!(reg.val, 255);
    }

    #[test]
    fn register_add_16bitwrap() {
        let mut reg: Register = Register::new("A", RegisterSize::Bits16);
        reg.set(65535);
        reg.add(1);
        assert_eq!(reg.val, 0);
    }

    #[test]
    fn register_sub_16bitwrap() {
        let mut reg: Register = Register::new("A", RegisterSize::Bits16);
        reg.set(0);
        reg.sub(1);
        assert_eq!(reg.val, 65535);
        reg.set(0);
        reg.sub(11);
        assert_eq!(reg.val, 65525);
    }
}
