use gb_asm::instructions::{
    subroutine::call::{Call, VectorSlot},
    Condition, Instruction,
};

pub fn call<C>(condition: C) -> Instruction
where
    C: Into<Option<Condition>>,
{
    Call::ConstantAddress(condition.into()).into()
}

pub fn call_vector(vector: VectorSlot) -> Instruction {
    Call::Vector(vector).into()
}
