use crate::{Execute, LoadValue, WriteValue};
use gb_asm::{instructions::bitwise::shift_right::*, Flag, Info, Pair};
use gb_hardware::Device;

impl Execute for ShiftRight {
    fn execute(&self, device: &mut Device) -> u8 {
        let value = self.target.load_value(device);

        // Store most-significant bit in case we determine this is an arithmetic shift later on.
        let msb = value & (1 << 7);
        let (mut result, carry) = value.overflowing_shr(1);

        if matches!(self.behavior, Behavior::Arithmetic) {
            result |= msb;
        }

        self.target.write_value(device, result);

        device.cpu.set(Flag::Zero, result == 0);
        device.cpu.set(Flag::Subtract, false);
        device.cpu.set(Flag::HalfCarry, false);
        device.cpu.set(Flag::Carry, carry);

        self.cycles().max()
    }
}

impl LoadValue for Target {
    type Value = u8;

    fn load_value(&self, device: &Device) -> Self::Value {
        match self {
            Self::Register(r) => device.cpu.get(r),
            Self::PointerValue => device.read_byte(device.cpu.get(Pair::HL)),
        }
    }
}

impl WriteValue for Target {
    type Value = u8;

    fn write_value(&self, device: &mut Device, value: Self::Value) {
        match self {
            Self::Register(r) => device.cpu.set(r, value),
            Self::PointerValue => device.write_byte(device.cpu.get(Pair::HL), value),
        }
    }
}
