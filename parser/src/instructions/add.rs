use gb_asm::{
    instructions::{math::add::*, Instruction},
    Pair, Register,
};

pub fn add_pair_to_hl(pair: Pair) -> Instruction {
    ToHLPair {
        source: ToHLPairSource::Pair(pair),
    }
    .into()
}

pub fn add_register_to_accumulator(source: Register) -> Instruction {
    ToAccumulator {
        source: ToAccumulatorSource::Register(source),
    }
    .into()
}

pub fn add_signed_constant_to_stack_pointer() -> Instruction {
    Add::ToStackPointer.into()
}

pub fn add_stack_pointer_to_hl() -> Instruction {
    ToHLPair {
        source: ToHLPairSource::StackPointer,
    }
    .into()
}

pub fn add_pointer_value_to_accumulator() -> Instruction {
    ToAccumulator {
        source: ToAccumulatorSource::PointerValue,
    }
    .into()
}

pub fn add_constant_to_accumulator() -> Instruction {
    ToAccumulator {
        source: ToAccumulatorSource::ConstantByte,
    }
    .into()
}
