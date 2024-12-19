use crate::Execute;
use gb_asm::{instructions::Instruction, Info};
use gb_hardware::Device;

pub mod bitwise;
pub mod jump;
pub mod load;
pub mod math;
pub mod misc;
pub mod stack;
pub mod subroutine;

impl Execute for Instruction {
    fn execute(&self, device: &mut Device) -> u8 {
        match self {
            Self::AddPlusCarry(inner) => inner.execute(device),
            Self::Add(inner) => inner.execute(device),
            Self::And(inner) => inner.execute(device),
            Self::Test(inner) => inner.execute(device),
            Self::Call(inner) => inner.execute(device),
            Self::ComplementCarryFlag(inner) => misc::complement_carry_flag(inner, device),
            Self::Compare(inner) => inner.execute(device),
            Self::ComplementAccumulator(inner) => misc::complement_accumulator(inner, device),
            Self::DecimalAdjustAccumulator(inner) => {
                misc::decimal_adjust_accumulator(inner, device)
            }
            Self::Decrement(inner) => inner.execute(device),
            Self::DisableInterrupts(inner) => misc::disable_interrupts(inner, device),
            Self::EnableInterrupts(inner) => misc::enable_interrupts(inner, device),
            Self::Halt(inner) => misc::halt(inner, device),
            Self::Increment(inner) => inner.execute(device),
            Self::Jump(inner) => inner.execute(device),
            Self::JumpRelative(inner) => inner.execute(device),
            Self::Load(inner) => inner.execute(device),
            Self::Nop(inner) => inner.cycles().max(),
            Self::Or(inner) => inner.execute(device),
            Self::Pop(inner) => inner.execute(device),
            Self::Push(inner) => inner.execute(device),
            Self::ResetBit(inner) => inner.execute(device),
            Self::Return(inner) => inner.execute(device),
            Self::Rotate(inner) => inner.execute(device),
            Self::Subtract(inner) => inner.execute(device),
            Self::SetCarryFlag(inner) => misc::set_carry_flag(inner, device),
            Self::SetBit(inner) => inner.execute(device),
            Self::ShiftLeft(inner) => inner.execute(device),
            Self::ShiftRight(inner) => inner.execute(device),
            Self::Stop(inner) => misc::stop(inner, device),
            Self::Swap(inner) => inner.execute(device),
            Self::Xor(inner) => inner.execute(device),
            Self::Prefix(inner) => inner.cycles().max(),
        }
    }
}
