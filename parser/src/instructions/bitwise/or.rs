use gb_asm::{
    instructions::{bitwise::or::*, Instruction},
    sources::ByteSource::*,
    Register,
};

pub fn or_register(register: Register) -> Instruction {
    Or {
        source: Register(register),
    }
    .into()
}

pub fn or_pointer_value() -> Instruction {
    Or {
        source: PointerValue,
    }
    .into()
}

pub fn or_constant() -> Instruction {
    Or {
        source: ConstantByte,
    }
    .into()
}
