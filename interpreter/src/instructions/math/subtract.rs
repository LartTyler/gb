use crate::{math::GbSub, Execute, LoadValue};
use gb_asm::{instructions::math::subtract::*, Flag, Info};
use gb_hardware::Device;

impl Execute for Subtract {
    fn execute(&self, device: &mut Device) -> u8 {
        let rhs = self.source.load_value(device);
        let carry = device.cpu.get(Flag::Carry);

        // To simplify things, we always call `sub_with_carry()`, but ignore the carry flag if this
        // isn't an SBC instruction.
        let result = device.cpu.a.sub_with_carry(rhs, self.with_carry && carry);

        device.cpu.set(Flag::Subtract, true);
        result.copy_to_cpu_flags(&mut device.cpu);

        self.cycles().max()
    }
}
