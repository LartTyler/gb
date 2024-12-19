use gb_asm::{
    instructions::{
        stack::{
            pop::{Source::*, *},
            push::Push,
        },
        Instruction,
    },
    Pair,
};

pub fn pop_pair(pair: Pair) -> Instruction {
    Pop { target: Pair(pair) }.into()
}

pub fn pop_accumulator_and_flags() -> Instruction {
    Pop {
        target: AccumulatorAndFlags,
    }
    .into()
}

pub fn push_pair(pair: Pair) -> Instruction {
    Push { source: Pair(pair) }.into()
}

pub fn push_accumulator_and_flags() -> Instruction {
    Push {
        source: AccumulatorAndFlags,
    }
    .into()
}
