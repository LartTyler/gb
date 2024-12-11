use crate::{cycles, instructions::Condition, Cycles, Info};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Call {
    Vector(Vector),
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
            Self::Vector(v) => write!(f, "RST {}", v.value()),
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
pub struct Vector(u8);

impl Vector {
    const ALLOWED_VALUES: &[u8] = &[0, 8, 10, 18, 20, 28, 30, 38];

    pub fn new(value: u8) -> Result<Self, &'static str> {
        if !Self::ALLOWED_VALUES.contains(&value) {
            Err("value is not a recognized jump vector")
        } else {
            Ok(Self(value))
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}
