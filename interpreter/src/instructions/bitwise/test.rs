use crate::{Execute, LoadValue, WriteValue};
use gb_asm::{
    instructions::bitwise::test::{
        Target::{self, *},
        *,
    },
    Flag, Info, Pair,
};
use gb_hardware::Device;

impl Execute for Test {
    fn execute(&self, device: &mut Device) -> u8 {
        let mask = 1 << self.position.value();
        let result = self.target.load_value(device) & mask;

        device.cpu.set(Flag::Zero, result == 0);
        device.cpu.set(Flag::Subtract, false);
        device.cpu.set(Flag::HalfCarry, true);

        self.cycles().max()
    }
}

impl LoadValue for Target {
    type Value = u8;

    fn load_value(&self, device: &Device) -> Self::Value {
        match self {
            Register(r) => device.cpu.get(r),
            PointerValue => device.read_byte(device.cpu.get(Pair::HL)),
        }
    }
}

impl WriteValue for Target {
    type Value = u8;

    fn write_value(&self, device: &mut Device, value: Self::Value) {
        match self {
            Register(r) => device.cpu.set(r, value),
            PointerValue => device.write_byte(device.cpu.get(Pair::HL), value),
        }
    }
}
