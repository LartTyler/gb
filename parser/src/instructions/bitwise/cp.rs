use gb_asm::{
    instructions::{math::cp::*, Instruction},
    Register,
};

pub fn cp_register(register: Register) -> Instruction {
    Compare::Register(register).into()
}

pub fn cp_pointer_value() -> Instruction {
    Compare::PointerValue.into()
}

pub fn cp_constant() -> Instruction {
    Compare::ConstantByte.into()
}
