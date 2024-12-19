use crate::{Execute, LoadValue, WriteValue};
use gb_asm::{instructions::bitwise::swap::Swap, Flag, Info};
use gb_hardware::Device;

impl Execute for Swap {
    fn execute(&self, device: &mut Device) -> u8 {
        let value = self.target.load_value(device).rotate_right(4);
        self.target.write_value(device, value);

        device.cpu.set(Flag::Zero, value == 0);
        device.cpu.set(Flag::Subtract, false);
        device.cpu.set(Flag::HalfCarry, false);
        device.cpu.set(Flag::Carry, false);

        self.cycles().max()
    }
}
