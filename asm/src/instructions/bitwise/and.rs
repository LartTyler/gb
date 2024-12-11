use crate::{cycles, Cycles, Info, Register};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("AND {source}")]
pub struct And {
    pub source: Source,
}

impl Info for And {
    fn bytes(&self) -> u8 {
        use Source::*;

        match self.source {
            Register(_) | PointerValue => 1,
            ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        use Source::*;

        match self.source {
            Register(_) => cycles!(1),
            ConstantByte | PointerValue => cycles!(2),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Source {
    #[display("{_0}")]
    Register(Register),
    #[display("d8")]
    ConstantByte,
    #[display("(HL)")]
    PointerValue,
}
