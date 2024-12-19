use gb_asm::{
    instructions::{math::subtract::*, Instruction},
    sources::ByteSource::{self, *},
    Register,
};

pub fn subtract_register(register: Register) -> Instruction {
    subtract(Register(register), false)
}

pub fn subtract_pointer_value() -> Instruction {
    subtract(PointerValue, false)
}

pub fn subtract_constant() -> Instruction {
    subtract(ConstantByte, false)
}

pub fn subtract_register_with_carry(register: Register) -> Instruction {
    subtract(Register(register), true)
}

pub fn subtract_pointer_value_with_carry() -> Instruction {
    subtract(PointerValue, true)
}

pub fn subtract_constant_with_carry() -> Instruction {
    subtract(ConstantByte, true)
}

pub fn subtract(source: ByteSource, with_carry: bool) -> Instruction {
    Subtract { source, with_carry }.into()
}
