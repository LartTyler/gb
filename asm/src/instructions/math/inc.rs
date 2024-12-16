use crate::{Cycles, Info, Pair, Register};
use derive_more::derive::Display;
use Target::*;

#[derive(Debug, Copy, Clone, Display)]
#[display("INC {target}")]
pub struct Increment {
    pub target: Target,
}

impl Info for Increment {
    fn bytes(&self) -> u8 {
        1
    }

    fn cycles(&self) -> Cycles {
        match self.target {
            Register(_) => Cycles::Fixed(1),
            Pair(_) | StackPointer => Cycles::Fixed(2),
            PointerValue => Cycles::Fixed(3),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Target {
    #[display("{_0}")]
    Register(Register),
    #[display("{_0}")]
    Pair(Pair),
    #[display("SP")]
    StackPointer,
    #[display("(HL)")]
    PointerValue,
}
