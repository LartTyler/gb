use gb_asm::{
    instructions::{
        bitwise::and::{Source::*, *},
        Instruction,
    },
    Register,
};

pub fn and_register(register: Register) -> Instruction {
    And {
        source: Register(register),
    }
    .into()
}

pub fn and_pointer_value() -> Instruction {
    And {
        source: PointerValue,
    }
    .into()
}

pub fn and_constant() -> Instruction {
    And {
        source: ConstantByte,
    }
    .into()
}
