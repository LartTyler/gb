use crate::{cycles, Bit, Cycles, Info, Register};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("BIT {position}, {target}")]
pub struct Test {
    pub position: Bit,
    pub target: Target,
}

impl Info for Test {
    fn bytes(&self) -> u8 {
        2
    }

    fn cycles(&self) -> Cycles {
        use Target::*;

        match self.target {
            PointerValue => cycles!(3),
            Register(_) => cycles!(2),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Target {
    #[display("(HL)")]
    PointerValue,

    #[display("{_0}")]
    Register(Register),
}
