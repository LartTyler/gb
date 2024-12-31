use crate::inspect::Inspect;
use gb_asm::instructions::math::adc::AddPlusCarry;
use gb_hardware::Device;

impl Inspect for AddPlusCarry {
    fn inspect(&self, device: &Device) -> String {
        format!("ADC A, {}", self.source.inspect(device))
    }
}
