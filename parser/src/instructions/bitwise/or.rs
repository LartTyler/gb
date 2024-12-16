use gb_asm::{
    instructions::{bitwise::or::*, Instruction},
    Register,
};

pub fn or_register(register: Register) -> Instruction {
    Or::Register(register).into()
}

pub fn or_pointer_value() -> Instruction {
    Or::PointerValue.into()
}

pub fn or_constant() -> Instruction {
    Or::ConstantByte.into()
}
