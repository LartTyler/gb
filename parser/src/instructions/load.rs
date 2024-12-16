use gb_asm::{
    instructions::{load::*, Instruction},
    Pair, Register,
};

pub fn load_into_pair(pair: Pair) -> Instruction {
    ToPair { target: pair }.into()
}

pub fn load_into_pair_pointer(pair: Pair) -> Instruction {
    ToPairPointer {
        target: ToPairPointerTarget::Pair(pair),
    }
    .into()
}

pub fn load_into_hl_incdec(action: Action) -> Instruction {
    ToPairPointer {
        target: ToPairPointerTarget::HLX(action),
    }
    .into()
}

pub fn load_into_stack_pointer() -> Instruction {
    ToStackPointer.into()
}

pub fn load_into_register_from_constant(register: Register) -> Instruction {
    ToRegister {
        target: register,
        source: ToRegisterSource::ConstantByte,
    }
    .into()
}

pub fn load_into_register_from_register(target: Register, source: Register) -> Instruction {
    ToRegister {
        target,
        source: ToRegisterSource::Register(source),
    }
    .into()
}

pub fn load_into_register_from_pointer_value(target: Register) -> Instruction {
    ToRegister {
        target,
        source: ToRegisterSource::PointerValue,
    }
    .into()
}

pub fn load_into_constant_pointer_from_stack_pointer() -> Instruction {
    ToConstantPointer {
        source: ToConstantPointerSource::StackPointer,
    }
    .into()
}

pub fn load_into_constant_pointer_from_accumulator() -> Instruction {
    ToConstantPointer {
        source: ToConstantPointerSource::Accumulator,
    }
    .into()
}

pub fn load_into_accumulator_from_pair_pointer(pair: Pair) -> Instruction {
    ToAccumulator {
        source: ToAccumulatorSource::PairPointer(pair),
    }
    .into()
}

pub fn load_into_accumulator_from_hlx(action: Action) -> Instruction {
    ToAccumulator {
        source: ToAccumulatorSource::HLX(action),
    }
    .into()
}

pub fn load_into_hl_pointer() -> Instruction {
    ToHLPointer {
        source: ToHLPointerSource::ConstantByte,
    }
    .into()
}

pub fn load_register_into_hl_pointer(register: Register) -> Instruction {
    ToHLPointer {
        source: ToHLPointerSource::Register(register),
    }
    .into()
}

pub fn load_into_high_constant_pointer() -> Instruction {
    ToHighConstantPointer.into()
}

pub fn load_into_highc_pointer() -> Instruction {
    ToHighC.into()
}

pub fn load_high_constant_pointer_into_accumulator() -> Instruction {
    ToAccumulator {
        source: ToAccumulatorSource::HighConstantPointer,
    }
    .into()
}

pub fn load_highc_pointer_into_accumulator() -> Instruction {
    ToAccumulator {
        source: ToAccumulatorSource::HighC,
    }
    .into()
}

pub fn load_constant_pointer_into_accumulator() -> Instruction {
    ToAccumulator {
        source: ToAccumulatorSource::ConstantPointer,
    }
    .into()
}

pub fn load_stack_pointer_plus_signed_constant_into_hl() -> Instruction {
    ToHL.into()
}
