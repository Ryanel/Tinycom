use num::{PrimInt, Unsigned};
use num_traits::*;
#[derive(Debug)]
pub struct Register<T: PrimInt + Unsigned + WrappingAdd> {
    pub value: T
}

impl<T: PrimInt + Unsigned + WrappingAdd> Register<T>  {
    pub fn add(&mut self, val: T) {
        let n: &T = &val;
        self.value = self.value.wrapping_add(n);
    }

    pub fn add_checked(&mut self, val: T) -> bool {
        let n: &T = &val;

        let overflows = self.value.checked_add(n);
        self.value = self.value.wrapping_add(n);

        overflows.is_none() // Return true if we overflow
    }
}