use crate::{Execute, LoadValue, WriteValue};
use gb_asm::{instructions::bitwise::shift_left::*, Flag, Info};
use gb_hardware::Device;

impl Execute for ShiftLeft {
    fn execute(&self, device: &mut Device) -> u8 {
        let (result, carry) = self.target.load_value(device).overflowing_shl(1);
        self.target.write_value(device, result);

        device.cpu.set(Flag::Zero, result == 0);
        device.cpu.set(Flag::Subtract, false);
        device.cpu.set(Flag::HalfCarry, false);
        device.cpu.set(Flag::Carry, carry);

        self.cycles().max()
    }
}
