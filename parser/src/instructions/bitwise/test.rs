use gb_asm::{
    instructions::{
        bitwise::test::{Source::*, *},
        Instruction,
    },
    Bit, Register,
};

pub fn test_register(position: Bit, register: Register) -> Instruction {
    Test {
        position,
        source: Register(register),
    }
    .into()
}

pub fn test_pointer_value(position: Bit) -> Instruction {
    Test {
        position,
        source: PointerValue,
    }
    .into()
}
