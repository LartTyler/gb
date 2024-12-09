use crate::{Cycles, Info, Register};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Source {
    Register(Register),
    ImmediateByte,
    HL,
}

#[derive(Debug, Copy, Clone)]
pub struct AddPlusCarry {
    pub source: Source,
}

impl Info for AddPlusCarry {
    fn bytes(&self) -> u8 {
        match self.source {
            Source::HL | Source::Register(_) => 1,
            Source::ImmediateByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        match self.source {
            Source::HL | Source::ImmediateByte => Cycles::Fixed(2),
            Source::Register(_) => Cycles::Fixed(1),
        }
    }
}

impl Display for AddPlusCarry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ADC A, ")?;

        match &self.source {
            Source::HL => write!(f, "(HL)"),
            Source::ImmediateByte => write!(f, "d8"),
            Source::Register(r) => r.fmt(f),
        }
    }
}
