use crate::{math::GbSub, Execute, LoadValue};
use gb_asm::{instructions::math::cp::*, Flag, Info};
use gb_hardware::Device;

impl Execute for Compare {
    fn execute(&self, device: &mut Device) -> u8 {
        let rhs = self.source.load_value(device);
        let result = device.cpu.a.sub(rhs);

        result.copy_to_cpu_flags(&mut device.cpu);
        device.cpu.set(Flag::Subtract, true);

        self.cycles().max()
    }
}
