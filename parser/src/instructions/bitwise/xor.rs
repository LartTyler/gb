use gb_asm::{
    instructions::{bitwise::xor::*, Instruction},
    Register,
};

pub fn xor_register(register: Register) -> Instruction {
    Xor::Register(register).into()
}

pub fn xor_pointer_value() -> Instruction {
    Xor::PointerValue.into()
}

pub fn xor_constant() -> Instruction {
    Xor::ConstantByte.into()
}
