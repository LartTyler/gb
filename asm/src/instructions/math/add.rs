use crate::{cycles, instructions::Instruction, Cycles, Info, Pair, Register};
use derive_more::derive::{Display, From};

#[derive(Debug, Copy, Clone, Display, From)]
pub enum Add {
    #[display("ADD {_0}")]
    ToAccumulator(ToAccumulator),
    #[display("ADD {_0}")]
    ToHLPair(ToHLPair),
    #[display("ADD SP, s8")]
    ToStackPointer,
}

impl Info for Add {
    fn bytes(&self) -> u8 {
        match self {
            Self::ToAccumulator(v) => v.bytes(),
            Self::ToHLPair(_) => 1,
            Self::ToStackPointer => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        match self {
            Self::ToAccumulator(v) => v.cycles(),
            Self::ToHLPair(_) => cycles!(2),
            Self::ToStackPointer => cycles!(4),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
#[display("A, {source}")]
pub struct ToAccumulator {
    pub source: ToAccumulatorSource,
}

impl Info for ToAccumulator {
    fn bytes(&self) -> u8 {
        use ToAccumulatorSource::*;

        match self.source {
            PointerValue | Register(_) => 1,
            ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        use ToAccumulatorSource::*;

        match self.source {
            Register(_) => Cycles::Fixed(1),
            ConstantByte | PointerValue => Cycles::Fixed(2),
        }
    }
}

impl From<ToAccumulator> for Instruction {
    fn from(value: ToAccumulator) -> Self {
        Add::from(value).into()
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum ToAccumulatorSource {
    #[display("{_0}")]
    Register(Register),
    #[display("(HL)")]
    PointerValue,
    #[display("d8")]
    ConstantByte,
}

#[derive(Debug, Copy, Clone, Display)]
#[display("HL, {source}")]
pub struct ToHLPair {
    pub source: ToHLPairSource,
}

impl From<ToHLPair> for Instruction {
    fn from(value: ToHLPair) -> Self {
        Add::from(value).into()
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum ToHLPairSource {
    #[display("{_0}")]
    Pair(Pair),
    #[display("SP")]
    StackPointer,
}
