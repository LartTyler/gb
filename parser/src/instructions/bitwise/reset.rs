use gb_asm::{
    instructions::{bitwise::reset::*, bitwise::test::Target::*, Instruction},
    Bit, Register,
};

pub fn reset_register(position: Bit, register: Register) -> Instruction {
    ResetBit {
        position,
        target: Register(register),
    }
    .into()
}

pub fn reset_pointer_value(position: Bit) -> Instruction {
    ResetBit {
        position,
        target: PointerValue,
    }
    .into()
}
