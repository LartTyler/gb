use crate::{cycles, Cycles, Info, Register};
use derive_more::derive::Display;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Rotate {
    pub direction: Direction,
    pub target: Target,
    pub behavior: Behavior,
}

impl Display for Rotate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R")?;

        match self.direction {
            Direction::Left => write!(f, "L")?,
            Direction::Right => write!(f, "R")?,
        }

        if self.behavior == Behavior::Cyclic {
            write!(f, "C")?;
        }

        write!(f, "{}", self.target)
    }
}

impl Info for Rotate {
    fn bytes(&self) -> u8 {
        if self.target == Target::Accumulator {
            1
        } else {
            2
        }
    }

    fn cycles(&self) -> Cycles {
        use Target::*;

        match self.target {
            Accumulator => cycles!(1),
            Register(_) => cycles!(2),
            PointerValue => cycles!(4),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Behavior {
    Carrying,
    Cyclic,
}

#[derive(Debug, Copy, Clone, Display, PartialEq, Eq)]
pub enum Target {
    #[display("A")]
    Accumulator,
    #[display(" {_0}")]
    Register(Register),
    #[display(" (HL)")]
    PointerValue,
}
