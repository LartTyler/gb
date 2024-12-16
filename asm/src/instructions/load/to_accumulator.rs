use super::{Action, Load};
use crate::{cycles, instructions::Instruction, Cycles, Info, Pair};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("A, {source}")]
pub struct ToAccumulator {
    pub source: ToAccumulatorSource,
}

impl Info for ToAccumulator {
    fn bytes(&self) -> u8 {
        use ToAccumulatorSource::*;

        match self.source {
            PairPointer(_) | HLX(_) | HighC => 1,
            HighConstantPointer => 2,
            ConstantPointer => 3,
        }
    }

    fn cycles(&self) -> Cycles {
        use ToAccumulatorSource::*;

        match self.source {
            PairPointer(_) | HLX(_) | HighC => cycles!(2),
            HighConstantPointer => cycles!(3),
            ConstantPointer => cycles!(4),
        }
    }
}

impl From<ToAccumulator> for Instruction {
    fn from(value: ToAccumulator) -> Self {
        Load::from(value).into()
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum ToAccumulatorSource {
    #[display("({_0})")]
    PairPointer(Pair),
    #[display("(d16)")]
    ConstantPointer,
    #[display("($FF00+d8)")]
    HighConstantPointer,
    #[display("(HL{_0})")]
    HLX(Action),
    #[display("($FF00+C)")]
    HighC,
}
