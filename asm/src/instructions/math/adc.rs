use crate::{Cycles, Info, Register};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("ADC A, {source}")]
pub struct AddPlusCarry {
    pub source: Source,
}

impl Info for AddPlusCarry {
    fn bytes(&self) -> u8 {
        match self.source {
            Source::PointerValue | Source::Register(_) => 1,
            Source::ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        match self.source {
            Source::PointerValue | Source::ConstantByte => Cycles::Fixed(2),
            Source::Register(_) => Cycles::Fixed(1),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Source {
    #[display("{_0}")]
    Register(Register),
    #[display("d8")]
    ConstantByte,
    #[display("(HL)")]
    PointerValue,
}
