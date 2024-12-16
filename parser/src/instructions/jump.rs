use gb_asm::instructions::{
    jump::{Jump, JumpRelative, Target},
    Condition, Instruction,
};

pub fn relative_jump<C>(condition: C) -> Instruction
where
    C: Into<Option<Condition>>,
{
    JumpRelative {
        condition: condition.into(),
    }
    .into()
}

pub fn jump<C>(condition: C) -> Instruction
where
    C: Into<Option<Condition>>,
{
    Jump {
        target: Target::ConstantAddress(condition.into()),
    }
    .into()
}

pub fn jump_to_pointer() -> Instruction {
    Jump {
        target: Target::Pointer,
    }
    .into()
}
