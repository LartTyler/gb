use crate::{cycles, instructions::Condition, Cycles, Info};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Return {
    Normal(Option<Condition>),
    EnableInterrupts,
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RET")?;

        match self {
            Self::EnableInterrupts => write!(f, "I"),
            Self::Normal(Some(cond)) => write!(f, " {cond}"),
            _ => Ok(()),
        }
    }
}

impl Info for Return {
    fn bytes(&self) -> u8 {
        1
    }

    fn cycles(&self) -> Cycles {
        match self {
            Self::Normal(Some(_)) => cycles!(2, 5),
            Self::Normal(None) => cycles!(4),
            Self::EnableInterrupts => cycles!(4),
        }
    }
}
