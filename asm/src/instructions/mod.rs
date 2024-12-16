use crate::{with_info_trait, Cycles};
use derive_more::derive::{Display, From};

pub mod bitwise;
pub mod jump;
pub mod load;
pub mod math;
pub mod misc;
pub mod stack;
pub mod subroutine;

with_info_trait!(
    #[derive(Debug, Display, Copy, Clone, From)]
    pub enum Instruction {
        AddPlusCarry(math::adc::AddPlusCarry),
        Add(math::add::Add),
        And(bitwise::and::And),
        Test(bitwise::test::Test),
        Call(subroutine::call::Call),
        ComplementCarryFlag(misc::ComplementCarryFlag),
        Compare(math::cp::Compare),
        ComplementAccumulator(misc::ComplementAccumulator),
        DecimalAdjustAccumulator(misc::DecimalAdjustAccumulator),
        Decrement(math::dec::Decrement),
        DisableInterrupts(misc::DisableInterrupts),
        EnableInterrupts(misc::EnableInterrupts),
        Halt(misc::Halt),
        Increment(math::inc::Increment),
        Jump(jump::Jump),
        JumpRelative(jump::JumpRelative),
        Load(load::Load),
        Nop(misc::Nop),
        Or(bitwise::or::Or),
        Pop(stack::pop::Pop),
        Push(stack::push::Push),
        ResetBit(bitwise::reset::ResetBit),
        Return(subroutine::ret::Return),
        Rotate(bitwise::rotate::Rotate),
        Subtract(math::subtract::Subtract),
        SetCarryFlag(misc::SetCarryFlag),
        SetBit(bitwise::set::SetBit),
        ShiftLeft(bitwise::shift_left::ShiftLeft),
        ShiftRight(bitwise::shift_right::ShiftRight),
        Stop(misc::Stop),
        Swap(bitwise::swap::Swap),
        Xor(bitwise::xor::Xor),
        Prefix(misc::Prefix),
    }
);

#[derive(Debug, Copy, Clone, Display)]
pub enum Condition {
    #[display("Z")]
    Zero,
    #[display("NZ")]
    NotZero,
    #[display("C")]
    Carry,
    #[display("NC")]
    NotCarry,
}
