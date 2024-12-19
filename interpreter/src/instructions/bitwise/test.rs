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

    fn load_value(&self, Device { cpu, memory }: &Device) -> Self::Value {
        match self {
            Register(r) => cpu.get(r),
            PointerValue => memory.read_byte(cpu.get(Pair::HL)),
        }
    }
}

impl WriteValue for Target {
    type Value = u8;

    fn write_value(&self, Device { cpu, memory }: &mut Device, value: Self::Value) {
        match self {
            Register(r) => cpu.set(r, value),
            PointerValue => memory.write_byte(cpu.get(Pair::HL), value),
        }
    }
}
