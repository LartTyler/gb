use crate::{cycles, Cycles, Info, Register};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Xor {
    Register(Register),
    PointerValue,
    ConstantByte,
}

impl Display for Xor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XOR ")?;

        match self {
            Self::Register(v) => write!(f, "{v}"),
            Self::PointerValue => write!(f, "(HL)"),
            Self::ConstantByte => write!(f, "d8"),
        }
    }
}

impl Info for Xor {
    fn bytes(&self) -> u8 {
        match self {
            Self::Register(_) | Self::PointerValue => 1,
            Self::ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        match self {
            Self::Register(_) => cycles!(1),
            Self::PointerValue | Self::ConstantByte => cycles!(2),
        }
    }
}
