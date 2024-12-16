use super::reset::Target;
use crate::{cycles, Bit, Cycles, Info};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("SET {position}, {target}")]
pub struct SetBit {
    pub position: Bit,
    pub target: Target,
}

impl Info for SetBit {
    fn bytes(&self) -> u8 {
        2
    }

    fn cycles(&self) -> Cycles {
        use Target::*;

        match self.target {
            Register(_) => cycles!(2),
            PointerValue => cycles!(4),
        }
    }
}
