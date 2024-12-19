use crate::{
    cycles,
    sources::ByteSource::{self, *},
    Cycles, Info,
};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Subtract {
    pub source: ByteSource,
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

        write!(f, " {}", self.source)
    }
}
