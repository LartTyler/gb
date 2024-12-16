use gb_asm::instructions::{subroutine::ret::Return, Condition, Instruction};

pub fn sub_return<C>(condition: C) -> Instruction
where
    C: Into<Option<Condition>>,
{
    Return::Normal(condition.into()).into()
}

pub fn sub_return_enable_interrupts() -> Instruction {
    Return::EnableInterrupts.into()
}
