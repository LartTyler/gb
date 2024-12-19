use gb_asm::{
    instructions::{bitwise::xor::*, Instruction},
    sources::ByteSource::*,
    Register,
};

pub fn xor_register(register: Register) -> Instruction {
    Xor {
        source: Register(register),
    }
    .into()
}

pub fn xor_pointer_value() -> Instruction {
    Xor {
        source: PointerValue,
    }
    .into()
}

pub fn xor_constant() -> Instruction {
    Xor {
        source: ConstantByte,
    }
    .into()
}
