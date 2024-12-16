use crate::{cycles, with_info_trait, with_simple_info, Cycles, Info, Pair, Register};
use derive_more::derive::{Display, From};

pub mod to_register;
pub use to_register::*;

pub mod to_accumulator;
pub use to_accumulator::*;

use super::Instruction;

with_info_trait!(
    #[derive(Debug, Copy, Clone, Display, From)]
    #[display("LD {_0}")]
    pub enum Load {
        /// LD r8, r8
        /// LD r8, d8
        /// LD r8, (HL)
        ToRegister(ToRegister),

        /// LD A, r8
        /// LD A, (r16)
        /// LD A, (HL+)
        /// LD A, (HL-)
        /// LD A, (d16)
        /// LD A, (C)
        /// LD A, ($FF00+d8)
        ToAccumulator(ToAccumulator),

        /// LD r16, d16
        ToPair(ToPair),

        /// LD (r16), A
        /// LD (HL+), A
        /// LD (HL-), A
        ToPairPointer(ToPairPointer),

        /// LD (HL), d8
        ToHLPointer(ToHLPointer),

        /// LD SP, HL
        ToStackPointer(ToStackPointer),

        /// LD ($FF00+C), A
        ToHighC(ToHighC),

        /// LD (d16), A
        ToConstantPointer(ToConstantPointer),

        /// LD ($FF00+d8), A
        ToHighConstantPointer(ToHighConstantPointer),

        /// LD HL, SP+s8
        ToHL(ToHL),
    }
);

#[derive(Debug, Copy, Clone, Display)]
pub enum Action {
    #[display("+")]
    Increment,
    #[display("-")]
    Decrement,
}

#[derive(Debug, Copy, Clone, Display)]
#[display("{target}, d16")]
pub struct ToPair {
    pub target: Pair,
}

with_simple_info!(ToPair => (3, 3));

impl From<ToPair> for Instruction {
    fn from(value: ToPair) -> Self {
        Load::from(value).into()
    }
}

#[derive(Debug, Copy, Clone, Display)]
#[display("(HL), {source}")]
pub struct ToHLPointer {
    pub source: ToHLPointerSource,
}

impl Info for ToHLPointer {
    fn bytes(&self) -> u8 {
        use ToHLPointerSource::*;

        match self.source {
            Register(_) => 1,
            ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        use ToHLPointerSource::*;

        match self.source {
            Register(_) => cycles!(2),
            ConstantByte => cycles!(3),
        }
    }
}

impl From<ToHLPointer> for Instruction {
    fn from(value: ToHLPointer) -> Self {
        Load::from(value).into()
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum ToHLPointerSource {
    #[display("{_0}")]
    Register(Register),
    #[display("d8")]
    ConstantByte,
}

#[derive(Debug, Copy, Clone, Display)]
#[display("{target}, A")]
pub struct ToPairPointer {
    pub target: ToPairPointerTarget,
}

with_simple_info!(ToPairPointer => (1, 2));

impl From<ToPairPointer> for Instruction {
    fn from(value: ToPairPointer) -> Self {
        Load::from(value).into()
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum ToPairPointerTarget {
    #[display("({_0})")]
    Pair(Pair),
    #[display("(HL{_0})")]
    HLX(Action),
}

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("SP, HL")]
    pub struct ToStackPointer => (1, 2);
}

impl From<ToStackPointer> for Instruction {
    fn from(value: ToStackPointer) -> Self {
        Load::from(value).into()
    }
}

#[derive(Debug, Copy, Clone, Display)]
#[display("(d16), {source}")]
pub struct ToConstantPointer {
    pub source: ToConstantPointerSource,
}

impl Info for ToConstantPointer {
    fn bytes(&self) -> u8 {
        3
    }

    fn cycles(&self) -> Cycles {
        use ToConstantPointerSource::*;

        match self.source {
            Accumulator => cycles!(4),
            StackPointer => cycles!(5),
        }
    }
}

impl From<ToConstantPointer> for Instruction {
    fn from(value: ToConstantPointer) -> Self {
        Load::from(value).into()
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum ToConstantPointerSource {
    #[display("A")]
    Accumulator,
    #[display("SP")]
    StackPointer,
}

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("($FF00+C), A")]
    pub struct ToHighC => (1, 2);
}

impl From<ToHighC> for Instruction {
    fn from(value: ToHighC) -> Self {
        Load::ToHighC(value).into()
    }
}

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("HL, SP+s8")]
    pub struct ToHL => (2, 3);
}

impl From<ToHL> for Instruction {
    fn from(value: ToHL) -> Self {
        Load::ToHL(value).into()
    }
}

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("($FF00+d8), A")]
    pub struct ToHighConstantPointer => (2, 3);
}

impl From<ToHighConstantPointer> for Instruction {
    fn from(value: ToHighConstantPointer) -> Self {
        Load::from(value).into()
    }
}
