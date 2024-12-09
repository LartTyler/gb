use crate::{Cycles, Info, Pair, Register};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Add {
    ToAccumulator(ToAccumulator),
    ToHLPair(ToHLPair),
    ToStackPointer,
}

impl Info for Add {
    fn bytes(&self) -> u8 {
        match self {
            Self::ToAccumulator(v) => v.bytes(),
            Self::ToHLPair(v) => v.bytes(),
            Self::ToStackPointer => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        match self {
            Self::ToAccumulator(v) => v.cycles(),
            Self::ToHLPair(v) => v.cycles(),
            Self::ToStackPointer => Cycles::Fixed(4),
        }
    }
}

impl Display for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ADD ")?;

        match self {
            Self::ToStackPointer => write!(f, "SP, s8"),
            Self::ToAccumulator(v) => write!(f, "{v}"),
            Self::ToHLPair(v) => write!(f, "HL, {v}"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ToAccumulator {
    pub source: ToAccumulatorSource,
}

impl Info for ToAccumulator {
    fn bytes(&self) -> u8 {
        use ToAccumulatorSource::*;

        match self.source {
            PointerValue | Register(_) => 1,
            ImmediateByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        use ToAccumulatorSource::*;

        match self.source {
            Register(_) => Cycles::Fixed(1),
            ImmediateByte | PointerValue => Cycles::Fixed(2),
        }
    }
}

impl Display for ToAccumulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A, {}", self.source)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ToAccumulatorSource {
    Register(Register),
    PointerValue,
    ImmediateByte,
}

impl Display for ToAccumulatorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PointerValue => write!(f, "(HL)"),
            Self::Register(r) => write!(f, "{r}"),
            Self::ImmediateByte => write!(f, "d8"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ToHLPair {
    pub source: ToHLPairSource,
}

impl Info for ToHLPair {
    fn bytes(&self) -> u8 {
        1
    }
    fn cycles(&self) -> Cycles {
        Cycles::Fixed(2)
    }
}

impl Display for ToHLPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HL, {}", self.source)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ToHLPairSource {
    Pair(Pair),
    StackPointer,
}

impl Display for ToHLPairSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StackPointer => write!(f, "SP"),
            Self::Pair(p) => write!(f, "{p}"),
        }
    }
}
