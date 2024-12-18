use crate::{Execute, LoadValue};
use gb_asm::{instructions::bitwise::and::*, Flag, Info};
use gb_hardware::Device;

impl Execute for And {
    fn execute(&self, device: &mut Device) -> u8 {
        let rhs = self.source.load_value(device);
        let result = device.cpu.a & rhs;

        device.cpu.set(Flag::Zero, result == 0);
        device.cpu.set(Flag::Subtract, false);
        device.cpu.set(Flag::HalfCarry, true);
        device.cpu.set(Flag::Carry, false);

        self.cycles().max()
    }
}
