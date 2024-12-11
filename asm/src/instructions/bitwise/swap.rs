use crate::{cycles, Info, Register};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Swap {
    Register(Register),
    PointerValue,
}

impl Info for Swap {
    fn bytes(&self) -> u8 {
        2
    }

    fn cycles(&self) -> crate::Cycles {
        match self {
            Self::Register(_) => cycles!(2),
            Self::PointerValue => cycles!(4),
        }
    }
}

impl Display for Swap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SWAP ")?;

        match self {
            Self::Register(v) => write!(f, "{v}"),
            Self::PointerValue => write!(f, "(HL)"),
        }
    }
}
