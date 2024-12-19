use crate::{Execute, LoadValue};
use gb_asm::{instructions::bitwise::or::*, Flag, Info};

impl Execute for Or {
    fn execute(&self, device: &mut gb_hardware::Device) -> u8 {
        let rhs = self.source.load_value(device);
        device.cpu.a |= rhs;

        device.cpu.set(Flag::Zero, device.cpu.a == 0);
        device.cpu.set(Flag::Subtract, false);
        device.cpu.set(Flag::HalfCarry, false);
        device.cpu.set(Flag::Carry, false);

        self.cycles().max()
    }
}
