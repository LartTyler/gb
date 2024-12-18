use super::dec::Target::{self, *};
use crate::{Cycles, Info};
use derive_more::derive::Display;

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
