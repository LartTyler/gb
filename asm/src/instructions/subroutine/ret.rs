use crate::{cycles, instructions::Condition, Cycles, Info};
use derive_more::derive::Display;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Display)]
pub enum Return {
    Normal(Normal),
    #[display("RETI")]
    EnableInterrupts,
}

impl Info for Return {
    fn bytes(&self) -> u8 {
        1
    }

    fn cycles(&self) -> Cycles {
        match self {
            Self::Normal(inner) => {
                if inner.condition.is_some() {
                    cycles!(2, 5)
                } else {
                    cycles!(4)
                }
            }
            Self::EnableInterrupts => cycles!(4),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Normal {
    pub condition: Option<Condition>,
}

impl Display for Normal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RET")?;

        if let Some(cond) = self.condition {
            write!(f, " {cond}")?;
        }

        Ok(())
    }
}
