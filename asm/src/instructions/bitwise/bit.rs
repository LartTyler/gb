use crate::{Cycles, Info, Register};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("BIT {position}, {source}")]
pub struct Bit {
    pub position: u8,
    pub source: Source,
}

impl Info for Bit {
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
