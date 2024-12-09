use crate::{with_info_trait, Cycles, Info};
use derive_more::derive::Display;

pub mod bitwise;
pub mod jump;
pub mod math;
pub mod misc;
pub mod subroutine;

with_info_trait!(
    #[derive(Debug, Display)]
    pub enum Instruction {
        AddPlusCarry(math::adc::AddPlusCarry),
        Add(math::add::Add),
        And(bitwise::and::And),
        Bit(bitwise::bit::Bit),
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
