use crate::{math::GbAdd, Execute, LoadValue};
use gb_asm::{instructions::math::adc::*, Flag, Info};
use gb_hardware::Device;

impl Execute for AddPlusCarry {
    fn execute(&self, device: &mut Device) -> u8 {
        let rhs = self.source.load_value(device);
        let carry = device.cpu.get(Flag::Carry);

        let result = device.cpu.a.add_with_carry(rhs, carry);
        device.cpu.a = result.value;

        result.copy_to_cpu_flags(&mut device.cpu);
        device.cpu.set(Flag::Subtract, false);

        self.cycles().max()
    }
}
