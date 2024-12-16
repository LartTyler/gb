use crate::{Bit, Cycles, Info, Register};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("BIT {position}, {source}")]
pub struct Test {
    pub position: Bit,
    pub source: Source,
}

impl Info for Test {
    fn bytes(&self) -> u8 {
        2
    }

    fn cycles(&self) -> Cycles {
        use Source::*;

        match self.source {
            PointerValue => Cycles::Fixed(3),
            Register(_) => Cycles::Fixed(2),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Source {
    #[display("(HL)")]
    PointerValue,

    #[display("{_0}")]
    Register(Register),
}
