use crate::{Cycles, Info, Register};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("CP {source}")]
pub struct Compare {
    pub source: Source,
}

impl Info for Compare {
    fn bytes(&self) -> u8 {
        use Source::*;

        match self.source {
            Register(_) | PointerValue => 1,
            ImmediateByte => 2,
        }
    }

    fn cycles(&self) -> crate::Cycles {
        use Source::*;

        match self.source {
            Register(_) => Cycles::Fixed(1),
            ImmediateByte | PointerValue => Cycles::Fixed(2),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Source {
    #[display("{_0}")]
    Register(Register),
    #[display("(HL)")]
    PointerValue,
    #[display("d8")]
    ImmediateByte,
}
