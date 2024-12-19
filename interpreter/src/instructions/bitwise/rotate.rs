use crate::{math::Operand, Execute, LoadValue, WriteValue};
use gb_asm::{
    instructions::bitwise::rotate::{Behavior::*, Direction::*, *},
    Flag, Info, Pair,
};
use gb_hardware::Device;

impl Execute for Rotate {
    fn execute(&self, device: &mut Device) -> u8 {
        let value = self.target.load_value(device);
        let carry = device.cpu.get(Flag::Carry);

        let (value, carry) = match self.direction {
            Left => match self.behavior {
                Carrying => value.carrying_shl(1, carry),
                Cyclic => {
                    let value = value.rotate_left(1);
                    (value, value & 1 != 0)
                }
            },
            Right => match self.behavior {
                Carrying => value.carrying_shr(1, carry),
                Cyclic => {
                    let value = value.rotate_right(1);
                    (value, value & (1 << (u8::BITS - 1)) != 0)
                }
            },
        };

        self.target.write_value(device, value);

        device.cpu.set(Flag::Subtract, false);
        device.cpu.set(Flag::HalfCarry, false);
        device.cpu.set(Flag::Carry, carry);

        if self.target == Target::Accumulator {
            device.cpu.set(Flag::Zero, false);
        } else {
            device.cpu.set(Flag::Zero, value == 0);
        }

        self.cycles().max()
    }
}

impl LoadValue for Target {
    type Value = u8;

    fn load_value(&self, Device { cpu, memory }: &Device) -> Self::Value {
        match self {
            Self::Accumulator => cpu.a,
            Self::Register(r) => cpu.get(r),
            Self::PointerValue => memory.read_byte(cpu.get(Pair::HL)),
        }
    }
}

impl WriteValue for Target {
    type Value = u8;

    fn write_value(&self, Device { cpu, memory }: &mut Device, value: Self::Value) {
        match self {
            Self::Accumulator => cpu.a = value,
            Self::Register(r) => cpu.set(r, value),
            Self::PointerValue => memory.write_byte(cpu.get(Pair::HL), value),
        }
    }
}
