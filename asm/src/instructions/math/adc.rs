use crate::{
    sources::ByteSource::{self, *},
    Cycles, Info,
};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("ADC A, {source}")]
pub struct AddPlusCarry {
    pub source: ByteSource,
}

impl Info for AddPlusCarry {
    fn bytes(&self) -> u8 {
        match self.source {
            PointerValue | Register(_) => 1,
            ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        match self.source {
            PointerValue | ConstantByte => Cycles::Fixed(2),
            Register(_) => Cycles::Fixed(1),
        }
    }
}
