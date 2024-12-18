use crate::Execute;
use gb_asm::{instructions::Instruction, Info};
use gb_hardware::Device;

pub mod bitwise;
pub mod math;
pub mod misc;
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
            Self::Increment(inner) => inner.execute(device),
            Self::Nop(nop) => nop.cycles().max(),
            _ => unimplemented!(),
        }
    }
}
