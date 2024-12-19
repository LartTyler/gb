use super::test::Target::{self, *};
use crate::{cycles, Bit, Cycles, Info};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("RES {position}, {target}")]
pub struct ResetBit {
    pub position: Bit,
    pub target: Target,
}

impl Info for ResetBit {
    fn bytes(&self) -> u8 {
        2
    }

    fn cycles(&self) -> Cycles {
        match self.target {
            Register(_) => cycles!(2),
            PointerValue => cycles!(4),
        }
    }
}
