use crate::{cycles, Cycles, Info, Register};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("SR{behavior} {target}")]
pub struct ShiftRight {
    pub target: Target,
    pub behavior: Behavior,
}

impl Info for ShiftRight {
    fn bytes(&self) -> u8 {
        2
    }

    fn cycles(&self) -> Cycles {
        use Target::*;

        match self.target {
            Register(_) => cycles!(2),
            PointerValue => cycles!(4),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Target {
    #[display("{_0}")]
    Register(Register),
    #[display("(HL)")]
    PointerValue,
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Behavior {
    #[display("A")]
    Arithmetic,
    #[display("L")]
    Logical,
}
