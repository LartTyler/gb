use crate::{math::GbAdd as _, Execute, LoadValue};
use gb_asm::{instructions::math::add::*, Flag, Info, Pair};
use gb_hardware::Device;

impl Execute for Add {
    fn execute(&self, device: &mut Device) -> u8 {
        match self {
            Self::ToAccumulator(inner) => {
                let rhs = inner.source.load_value(device);
                let result = device.cpu.a.add(rhs);
                result.copy_to_cpu_flags(&mut device.cpu);
            }
            Self::ToHLPair(inner) => {
                let rhs = inner.source.load_value(device);
                let result = device.cpu.get(Pair::HL).add(rhs);
                result.copy_to_cpu_flags(&mut device.cpu);
            }
            Self::ToStackPointer => {
                // It's important that we immediately cast the byte to an i8, otherwise the
                // following cast to a u16 won't saturate during expansion.
                let rhs = device.read_byte(device.cpu.program_counter) as i8;
                let result = device.cpu.stack_pointer.add(rhs as u16);
                result.copy_to_cpu_flags(&mut device.cpu);
            }
        };

        device.cpu.set(Flag::Subtract, false);

        self.cycles().max()
    }
}

impl LoadValue for ToHLPairSource {
    type Value = u16;

    fn load_value(&self, Device { cpu, .. }: &Device) -> Self::Value {
        match self {
            Self::Pair(p) => cpu.get(p),
            Self::StackPointer => cpu.stack_pointer,
        }
    }
}
