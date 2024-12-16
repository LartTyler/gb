use gb_asm::{
    instructions::{
        bitwise::shift_right::{Behavior::*, Target::*, *},
        Instruction,
    },
    Register,
};

pub fn arithmetic_shift_right_register(register: Register) -> Instruction {
    shift(Register(register), Arithmetic)
}

pub fn arithmetic_shift_right_pointer_value() -> Instruction {
    shift(PointerValue, Arithmetic)
}

pub fn logical_shift_right_register(register: Register) -> Instruction {
    shift(Register(register), Logical)
}

pub fn logical_shift_right_pointer_value() -> Instruction {
    shift(PointerValue, Logical)
}

fn shift(target: Target, behavior: Behavior) -> Instruction {
    ShiftRight { target, behavior }.into()
}
