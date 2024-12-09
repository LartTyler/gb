use crate::{Cycles, Info, Register};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct And {
    pub source: Source,
}

impl Display for And {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AND {}", self.source)
    }
}

impl Info for And {
    fn bytes(&self) -> u8 {
        use Source::*;

        match self.source {
            Register(_) | PointerValue => 1,
            ImmediateByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        use Source::*;

        match self.source {
            Register(_) => Cycles::Fixed(1),
            ImmediateByte | PointerValue => Cycles::Fixed(2),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Source {
    Register(Register),
    ImmediateByte,
    PointerValue,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Register(v) => write!(f, "{v}"),
            Self::ImmediateByte => write!(f, "d8"),
            Self::PointerValue => write!(f, "(HL)"),
        }
    }
}
