use super::shift_right::Target::{self, *};
use crate::{cycles, Cycles, Info};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("SLA {target}")]
pub struct ShiftLeft {
    pub target: Target,
}

impl Info for ShiftLeft {
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
