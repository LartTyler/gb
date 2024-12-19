use gb_asm::{
    instructions::{
        bitwise::{swap::*, test::Target::*},
        Instruction,
    },
    Register,
};

pub fn swap_register(register: Register) -> Instruction {
    Swap {
        target: Register(register),
    }
    .into()
}

pub fn swap_pointer_value() -> Instruction {
    Swap {
        target: PointerValue,
    }
    .into()
}
