use crate::{with_info_trait, Cycles, Info, Pair, Register};
use derive_more::derive::Display;

with_info_trait!(
    #[derive(Debug, Copy, Clone, Display)]
    #[display("LD {_0}")]
    pub enum Load {
        ToRegister(ToRegister),
        ToAccumulator(ToAccumulator),
    }
);

#[derive(Debug, Copy, Clone, Display)]
#[display("{target}, {source}")]
pub struct ToRegister {
    pub target: Register,
    pub source: ToRegisterSource,
}

impl Info for ToRegister {
    fn bytes(&self) -> u8 {
        use ToRegisterSource::*;

        match self.source {
            Register(_) | PointerValue => 1,
            ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        use ToRegisterSource::*;

        match self.source {
            Register(_) => Cycles::Fixed(1),
            PointerValue | ConstantByte => Cycles::Fixed(2),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum ToRegisterSource {
    #[display("{_0}")]
    Register(Register),
    #[display("d8")]
    ConstantByte,
    #[display("(HL)")]
    PointerValue,
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
            PairPointer(_) | HLX(_) => 1,
            ConstantPointer => 3,
        }
    }

    fn cycles(&self) -> Cycles {
        use ToAccumulatorSource::*;

        match self.source {
            PairPointer(_) | HLX(_) => Cycles::Fixed(2),
            ConstantPointer => Cycles::Fixed(4),
        }
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum ToAccumulatorSource {
    #[display("({_0})")]
    PairPointer(Pair),
    #[display("(d16)")]
    ConstantPointer,
    #[display("(HL{_0})")]
    HLX(Action),
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Action {
    #[display("+")]
    Increment,
    #[display("-")]
    Decrement,
}
