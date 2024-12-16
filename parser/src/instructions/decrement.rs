use gb_asm::{
    instructions::{
        math::dec::{Target::*, *},
        Instruction,
    },
    Pair, Register,
};

pub fn decrement_pair(pair: Pair) -> Instruction {
    Decrement { target: Pair(pair) }.into()
}

pub fn decrement_register(register: Register) -> Instruction {
    Decrement {
        target: Register(register),
    }
    .into()
}

pub fn decrement_stack_pointer() -> Instruction {
    Decrement {
        target: StackPointer,
    }
    .into()
}

pub fn decrement_pointer_value() -> Instruction {
    Decrement {
        target: PointerValue,
    }
    .into()
}
