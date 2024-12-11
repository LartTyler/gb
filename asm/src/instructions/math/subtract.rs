use crate::{cycles, Cycles, Info, Register};
use derive_more::derive::Display;
use std::fmt::Display;

use Source::*;

#[derive(Debug, Copy, Clone)]
pub struct Subtract {
    pub source: Source,
    pub with_carry: bool,
}

impl Info for Subtract {
    fn bytes(&self) -> u8 {
        match self.source {
            Register(_) | PointerValue => 1,
            ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        match self.source {
            Register(_) => cycles!(1),
            PointerValue | ConstantByte => cycles!(2),
        }
    }
}

impl Display for Subtract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.with_carry {
            write!(f, "SBC")?;
        } else {
            write!(f, "SUB")?;
        }

        write!(f, "{}", self.source)
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Source {
    #[display("{_0}")]
    Register(Register),
    #[display("(HL)")]
    PointerValue,
    #[display("(d8)")]
    ConstantByte,
}
