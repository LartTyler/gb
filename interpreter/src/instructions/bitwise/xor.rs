use crate::{Execute, LoadValue};
use gb_asm::{instructions::bitwise::xor::*, Flag, Info};
use gb_hardware::Device;

impl Execute for Xor {
    fn execute(&self, device: &mut Device) -> u8 {
        let rhs = self.source.load_value(device);
        device.cpu.a ^= rhs;

        device.cpu.set(Flag::Zero, device.cpu.a == 0);
        device.cpu.set(Flag::Subtract, false);
        device.cpu.set(Flag::HalfCarry, false);
        device.cpu.set(Flag::Carry, false);

        self.cycles().max()
    }
}
