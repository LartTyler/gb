use gb_asm::{
    instructions::{bitwise::shift_left::*, Instruction},
    Register,
};

pub fn shift_left_register(register: Register) -> Instruction {
    ShiftLeft::Register(register).into()
}

pub fn shift_left_pointer_value() -> Instruction {
    ShiftLeft::PointerValue.into()
}
