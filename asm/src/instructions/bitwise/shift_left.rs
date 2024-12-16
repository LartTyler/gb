use crate::{cycles, Cycles, Info, Register};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum ShiftLeft {
    Register(Register),
    PointerValue,
}

impl Display for ShiftLeft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SLA ")?;

        match self {
            Self::Register(v) => write!(f, "{v}"),
            Self::PointerValue => write!(f, "(HL)"),
        }
    }
}

impl Info for ShiftLeft {
    fn bytes(&self) -> u8 {
        2
    }

    fn cycles(&self) -> Cycles {
        match self {
            Self::Register(_) => cycles!(2),
            Self::PointerValue => cycles!(4),
        }
    }
}
