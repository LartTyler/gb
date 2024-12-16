use gb_asm::{
    instructions::{bitwise::swap::*, Instruction},
    Register,
};

pub fn swap_register(register: Register) -> Instruction {
    Swap::Register(register).into()
}

pub fn swap_pointer_value() -> Instruction {
    Swap::PointerValue.into()
}
