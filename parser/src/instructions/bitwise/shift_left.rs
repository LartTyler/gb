use gb_asm::{
    instructions::{
        bitwise::{shift_left::*, shift_right::Target::*},
        Instruction,
    },
    Register,
};

pub fn shift_left_register(register: Register) -> Instruction {
    ShiftLeft {
        target: Register(register),
    }
    .into()
}

pub fn shift_left_pointer_value() -> Instruction {
    ShiftLeft {
        target: PointerValue,
    }
    .into()
}
