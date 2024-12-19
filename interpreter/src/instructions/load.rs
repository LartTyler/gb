use crate::{Execute, LoadValue};
use gb_asm::{instructions::load::*, Info, Pair};
use gb_hardware::Device;

impl Execute for Load {
    fn execute(&self, device: &mut Device) -> u8 {
        match self {
            Self::ToRegister(inner) => {
                let value = inner.source.load_value(device);
                device.cpu.set(inner.target, value);
            }
            Self::ToAccumulator(inner) => {
                let value = inner.source.load_value(device);
                device.cpu.a = value;

                if let ToAccumulatorSource::HLX(action) = inner.source {
                    let pointer = device.cpu.get(Pair::HL);
                    device.cpu.set(Pair::HL, action.apply(pointer));
                }
            }
            Self::ToPair(inner) => {
                let value = device.memory.read_word(device.cpu.program_counter);
                device.cpu.set(inner.target, value);
            }
            Self::ToPairPointer(inner) => match inner.target {
                ToPairPointerTarget::HLX(action) => {
                    let pointer = device.cpu.get(Pair::HL);
                    device.memory.write_byte(pointer, device.cpu.a);

                    device.cpu.set(Pair::HL, action.apply(pointer));
                }
                ToPairPointerTarget::Pair(p) => {
                    let pointer = device.cpu.get(p);
                    device.memory.write_byte(pointer, device.cpu.a);
                }
            },
            Self::ToHLPointer(inner) => {
                let value = inner.source.load_value(device);
                device.memory.write_byte(device.cpu.get(Pair::HL), value);
            }
            Self::ToStackPointer(inner) => {
                device.cpu.stack_pointer = inner.source.load_value(device);
            }
            Self::ToHighC(_) => {
                let address = 0xFF00 + device.cpu.c as u16;
                device.memory.write_byte(address, device.cpu.a);
            }
            Self::ToConstantPointer(inner) => {
                let pointer = device.memory.read_word(device.cpu.program_counter);

                match inner.source {
                    ToConstantPointerSource::Accumulator => {
                        device.memory.write_byte(pointer, device.cpu.a);
                    }
                    ToConstantPointerSource::StackPointer => {
                        let [high, low] = device.cpu.stack_pointer.to_be_bytes();
                        device.memory.write_byte(pointer, high);
                        device.memory.write_byte(pointer + 1, low);
                    }
                };
            }
            Self::ToHighConstantPointer(_) => {
                let address = 0xFF00 + device.memory.read_byte(device.cpu.program_counter) as u16;
                device.memory.write_byte(address, device.cpu.a);
            }
            Self::ToHL(_) => {
                let offset = device.memory.read_byte(device.cpu.program_counter) as i8;
                let value = device.cpu.stack_pointer.wrapping_add(offset as u16);

                device.cpu.set(Pair::HL, value);
            }
        };

        self.cycles().max()
    }
}

impl LoadValue for ToAccumulatorSource {
    type Value = u8;

    fn load_value(&self, Device { cpu, memory }: &Device) -> Self::Value {
        match self {
            Self::HighC => memory.read_byte(0xFF00 + cpu.c as u16),
            Self::HighConstantPointer => {
                let value = memory.read_byte(cpu.program_counter);
                memory.read_byte(0xFF00 + value as u16)
            }
            Self::HLX(_) => memory.read_byte(cpu.get(Pair::HL)),
            Self::ConstantPointer => {
                let address = memory.read_byte(cpu.program_counter);
                memory.read_byte(address)
            }
            Self::PairPointer(p) => memory.read_byte(cpu.get(p)),
        }
    }
}

impl LoadValue for ToHLPointerSource {
    type Value = u8;

    fn load_value(&self, Device { cpu, memory }: &Device) -> Self::Value {
        match self {
            Self::ConstantByte => memory.read_byte(cpu.program_counter),
            Self::Register(r) => cpu.get(r),
        }
    }
}

impl LoadValue for ToStackPointerSource {
    type Value = u16;

    fn load_value(&self, Device { cpu, memory }: &Device) -> Self::Value {
        match self {
            Self::HL => cpu.get(Pair::HL),
            Self::ConstantWord => memory.read_word(cpu.program_counter),
        }
    }
}
