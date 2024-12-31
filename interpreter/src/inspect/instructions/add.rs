use crate::inspect::Inspect;
use gb_asm::instructions::math::add::ToHLPairSource;
use gb_hardware::Device;

impl Inspect for ToHLPairSource {
    fn inspect(&self, device: &Device) -> String {
        match self {}
    }
}
