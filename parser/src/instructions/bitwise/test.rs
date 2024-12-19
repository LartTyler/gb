use gb_asm::{
    instructions::{
        bitwise::test::{Target::*, *},
        Instruction,
    },
    Bit, Register,
};

pub fn test_register(position: Bit, register: Register) -> Instruction {
    Test {
        position,
        target: Register(register),
    }
    .into()
}

pub fn test_pointer_value(position: Bit) -> Instruction {
    Test {
        position,
        target: PointerValue,
    }
    .into()
}
