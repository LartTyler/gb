use gb_asm::{
    instructions::{math::adc::AddPlusCarry, Instruction},
    sources::ByteSource::*,
    Register,
};

pub fn add_register_to_accumulator_plus_carry(register: Register) -> Instruction {
    AddPlusCarry {
        source: Register(register),
    }
    .into()
}

pub fn add_constant_to_accumulator_plus_carry() -> Instruction {
    AddPlusCarry {
        source: ConstantByte,
    }
    .into()
}

pub fn add_pointer_value_to_accumulator_plus_carry() -> Instruction {
    AddPlusCarry {
        source: PointerValue,
    }
    .into()
}
