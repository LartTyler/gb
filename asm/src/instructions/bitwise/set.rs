use super::reset::Target;
use crate::{Bit, Cycles, Info};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("SET {bit}, {target}")]
pub struct SetBit {
    pub bit: Bit,
    pub target: Target,
}

impl Info for SetBit {
    fn bytes(&self) -> u8 {
        2
    }

    fn cycles(&self) -> Cycles {
        use Target::*;

        match self.target {}
    }
}
