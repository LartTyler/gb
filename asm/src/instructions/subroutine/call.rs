use crate::{cycles, instructions::Condition, Cycles, Info};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Call {
    Vector(VectorSlot),
    ConstantAddress(Option<Condition>),
}

impl Info for Call {
    fn bytes(&self) -> u8 {
        match self {
            Self::Vector(_) => 1,
            Self::ConstantAddress(_) => 3,
        }
    }

    fn cycles(&self) -> Cycles {
        match self {
            Self::Vector(_) => cycles!(4),
            Self::ConstantAddress(cond) => {
                if cond.is_some() {
                    cycles!(3, 6)
                } else {
                    cycles!(6)
                }
            }
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vector(v) => write!(f, "RST {}", *v as u8),
            Self::ConstantAddress(cond) => {
                write!(f, "CALL ")?;

                if let Some(cond) = cond {
                    write!(f, "{cond}")?;
                }

                write!(f, "d16")
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u16)]
/// Represents the "restart vector" slots present at the start of ROM. Can be cast to a `u8` in
/// order to access the address represented by each slot.
pub enum VectorSlot {
    Zero = 0,
    One = 8,
    Two = 10,
    Three = 18,
    Four = 20,
    Five = 28,
    Six = 30,
    Seven = 38,
}
