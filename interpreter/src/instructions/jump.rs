use crate::{Execute, LoadValue};
use gb_asm::{
    instructions::jump::{Target::*, *},
    Info, Pair,
};
use gb_hardware::Device;

impl Execute for Jump {
    fn execute(&self, device: &mut Device) -> u8 {
        if let Some(cond) = self.target.get_condition() {
            if !cond.test(device.cpu.flags) {
                return self.cycles().min();
            }
        }

        let address = self.target.load_value(device);
        device.cpu.program_counter = address;

        self.cycles().max()
    }
}

impl LoadValue for Target {
    type Value = u16;

    fn load_value(&self, Device { cpu, memory }: &Device) -> Self::Value {
        match self {
            Pointer => cpu.get(Pair::HL),
            ConstantAddress(_) => memory.read_word(cpu.program_counter),
        }
    }
}

impl Execute for JumpRelative {
    fn execute(&self, Device { cpu, memory }: &mut Device) -> u8 {
        if let Some(cond) = self.condition {
            if cond.test(cpu.flags) {
                return self.cycles().min();
            }
        }

        // The constant byte used by JR is a two's complement signed value. Since we need to expand
        // the value to a u16 in order to add it to PC, we need to cast it to an i8 when before we
        // use it, otherwise it won't saturate properly when expanding to a u16.
        let offset = memory.read_byte(cpu.program_counter) as i8;

        // We shift PC one byte forward during execution in order to simplify reading constant
        // values for instructions that need it. In this case, however, JR offsets PC starting at
        // the position _after_ the full instruction (including it's constant value). So, we need
        // to add one before adding our offset to ensure we end up in the correct place.
        // See https://rgbds.gbdev.io/docs/v0.8.0/gbz80.7#JR_n16
        cpu.program_counter = cpu
            .program_counter
            .wrapping_add(1)
            .wrapping_add(offset as u16);

        self.cycles().max()
    }
}
