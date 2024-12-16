use crate::{cycles, Cycles, Info, Register};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Compare {
    Register(Register),
    PointerValue,
    ConstantByte,
}

impl Display for Compare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CP ")?;

        match self {
            Self::Register(v) => write!(f, "{v}"),
            Self::PointerValue => write!(f, "(HL)"),
            Self::ConstantByte => write!(f, "d8"),
        }
    }
}

impl Info for Compare {
    fn bytes(&self) -> u8 {
        match self {
            Self::Register(_) | Self::PointerValue => 1,
            Self::ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        match self {
            Self::Register(_) => cycles!(1),
            Self::ConstantByte | Self::PointerValue => cycles!(2),
        }
    }
}
