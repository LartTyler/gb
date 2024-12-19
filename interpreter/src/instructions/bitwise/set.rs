use crate::{Execute, LoadValue, WriteValue};
use gb_asm::{instructions::bitwise::set::*, Info};
use gb_hardware::Device;

impl Execute for SetBit {
    fn execute(&self, device: &mut Device) -> u8 {
        let mask = 1 << self.position.value();
        let result = self.target.load_value(device) | mask;
        self.target.write_value(device, result);

        self.cycles().max()
    }
}
