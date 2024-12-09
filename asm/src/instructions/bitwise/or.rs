use crate::{cycles, Cycles, Info, Register};
use derive_more::derive::Display;

use Source::*;

#[derive(Debug, Copy, Clone, Display)]
#[display("OR {source}")]
pub struct Or {
    pub source: Source,
}

impl Info for Or {
    fn bytes(&self) -> u8 {
        match self.source {
            Register(_) | HLPointer => 1,
            ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        match self.source {
            Register(_) => cycles!(1),
            ConstantByte | HLPointer => cycles!(2),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Source {
    #[display("{_0}")]
    Register(Register),
    #[display("(HL)")]
    HLPointer,
    #[display("d8")]
    ConstantByte,
}
