use gb_asm::{
    instructions::{
        bitwise::rotate::{Behavior::*, Direction::*, Target::*, *},
        Instruction,
    },
    Register,
};

pub fn cyclic_rotate_left_accumulator() -> Instruction {
    rotate(Left, Accumulator, Cyclic)
}

pub fn cyclic_rotate_left_register(register: Register) -> Instruction {
    rotate(Left, Register(register), Cyclic)
}

pub fn cyclic_rotate_left_pointer_value() -> Instruction {
    rotate(Left, PointerValue, Cyclic)
}

pub fn carrying_rotate_left_accumulator() -> Instruction {
    rotate(Left, Accumulator, Carrying)
}

pub fn carrying_rotate_left_register(register: Register) -> Instruction {
    rotate(Left, Register(register), Carrying)
}

pub fn carrying_rotate_left_pointer_value() -> Instruction {
    rotate(Left, PointerValue, Carrying)
}

pub fn cyclic_rotate_right_accumulator() -> Instruction {
    rotate(Right, Accumulator, Cyclic)
}

pub fn cyclic_rotate_right_register(register: Register) -> Instruction {
    rotate(Right, Register(register), Cyclic)
}

pub fn cyclic_rotate_right_pointer_value() -> Instruction {
    rotate(Right, PointerValue, Cyclic)
}

pub fn carrying_rotate_right_accumulator() -> Instruction {
    rotate(Right, Accumulator, Carrying)
}

pub fn carrying_rotate_right_register(register: Register) -> Instruction {
    rotate(Right, Register(register), Carrying)
}

pub fn carrying_rotate_right_pointer_value() -> Instruction {
    rotate(Right, PointerValue, Carrying)
}

#[inline]
fn rotate(direction: Direction, target: Target, behavior: Behavior) -> Instruction {
    Rotate {
        direction,
        target,
        behavior,
    }
    .into()
}
