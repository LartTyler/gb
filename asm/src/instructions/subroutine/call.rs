use crate::{instructions::Condition, Cycles, Info};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Call {
    pub condition: Option<Condition>,
}

impl Info for Call {
    fn bytes(&self) -> u8 {
        3
    }

    fn cycles(&self) -> Cycles {
        Cycles::Variable(3, 6)
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CALL ")?;

        if let Some(cond) = self.condition {
            write!(f, "{cond}")?;
        }

        write!(f, "d16")
    }
}
