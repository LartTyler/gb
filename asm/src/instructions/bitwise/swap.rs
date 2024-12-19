use super::test::Target::{self, *};
use crate::{cycles, Info};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("SWAP {target}")]
pub struct Swap {
    pub target: Target,
}

impl Info for Swap {
    fn bytes(&self) -> u8 {
        2
    }

    fn cycles(&self) -> crate::Cycles {
        match self.target {
            Register(_) => cycles!(2),
            PointerValue => cycles!(4),
        }
    }
}
