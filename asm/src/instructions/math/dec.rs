use crate::{Cycles, Info, Pair, Register};
use derive_more::derive::Display;
use Source::*;

#[derive(Debug, Copy, Clone, Display)]
#[display("DEC {source}")]
pub struct Decrement {
    pub source: Source,
}

impl Info for Decrement {
    fn bytes(&self) -> u8 {
        1
    }

    fn cycles(&self) -> Cycles {
        match self.source {
            Register(_) => Cycles::Fixed(1),
            Pair(_) | StackPointer => Cycles::Fixed(2),
            PointerValue => Cycles::Fixed(3),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Source {
    #[display("{_0}")]
    Register(Register),
    #[display("{_0}")]
    Pair(Pair),
    #[display("SP")]
    StackPointer,
    #[display("(HL)")]
    PointerValue,
}
