use gb_asm::{
    instructions::{bitwise::reset::Target::*, bitwise::set::*, Instruction},
    Bit, Register,
};

pub fn set_register(position: Bit, register: Register) -> Instruction {
    SetBit {
        position,
        target: Register(register),
    }
    .into()
}

pub fn set_pointer_value(position: Bit) -> Instruction {
    SetBit {
        position,
        target: PointerValue,
    }
    .into()
}
