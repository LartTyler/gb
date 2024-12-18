use gb_asm::{
    instructions::{math::cp::*, Instruction},
    sources::ByteSource::*,
    Register,
};

pub fn compare_register(register: Register) -> Instruction {
    Compare {
        source: Register(register),
    }
    .into()
}

pub fn compare_pointer_value() -> Instruction {
    Compare {
        source: PointerValue,
    }
    .into()
}

pub fn compare_constant() -> Instruction {
    Compare {
        source: ConstantByte,
    }
    .into()
}
