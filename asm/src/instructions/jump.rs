use super::Condition;
use crate::{Cycles, Info};
use derive_more::derive::Display;
use std::fmt::Display;
use Target::*;

#[derive(Debug, Copy, Clone, Display)]
#[display("JP {target}")]
pub struct Jump {
    pub target: Target,
}

impl Info for Jump {
    fn bytes(&self) -> u8 {
        match self.target {
            Pointer => 1,
            ConstantAddress(_) => 3,
        }
    }

    fn cycles(&self) -> Cycles {
        match self.target {
            Pointer => Cycles::Fixed(1),
            ConstantAddress(cond) if cond.is_some() => Cycles::Variable(3, 4),
            ConstantAddress(_) => Cycles::Fixed(4),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Target {
    Pointer,
    ConstantAddress(Option<Condition>),
}

impl Target {
    pub fn get_condition(&self) -> Option<&Condition> {
        match self {
            Self::ConstantAddress(cond) => cond.as_ref(),
            _ => None,
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pointer => write!(f, "HL"),
            ConstantAddress(c) => {
                if let Some(c) = c {
                    write!(f, "{c}, ")?;
                }

                write!(f, "d16")
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct JumpRelative {
    pub condition: Option<Condition>,
}

impl Info for JumpRelative {
    fn bytes(&self) -> u8 {
        2
    }

    fn cycles(&self) -> Cycles {
        if self.condition.is_some() {
            Cycles::Variable(3, 4)
        } else {
            Cycles::Fixed(3)
        }
    }
}

impl Display for JumpRelative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JR ")?;

        if let Some(cond) = self.condition {
            write!(f, "{cond}, ")?;
        }

        write!(f, "s8")
    }
}
