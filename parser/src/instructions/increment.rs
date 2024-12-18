use gb_asm::{
    instructions::{math::dec::Target::*, math::inc::*, Instruction},
    Pair, Register,
};

pub fn increment_pair(pair: Pair) -> Instruction {
    Increment { target: Pair(pair) }.into()
}

pub fn increment_register(register: Register) -> Instruction {
    Increment {
        target: Register(register),
    }
    .into()
}

pub fn increment_stack_pointer() -> Instruction {
    Increment {
        target: StackPointer,
    }
    .into()
}

pub fn increment_pointer_value() -> Instruction {
    Increment {
        target: PointerValue,
    }
    .into()
}
