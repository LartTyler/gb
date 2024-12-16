use crate::{cycles, Bit, Cycles, Info, Register};
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
            Target::Register(_) => cycles!(2),
            Target::PointerValue => cycles!(4),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Target {
    #[display("{_0}")]
    Register(Register),
    #[display("(HL)")]
    PointerValue,
}
