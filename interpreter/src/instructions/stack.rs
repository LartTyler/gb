use crate::Execute;
use gb_asm::{
    instructions::stack::{
        pop::{Source::*, *},
        push::*,
    },
    Info,
};
use gb_hardware::Device;

impl Execute for Pop {
    fn execute(&self, Device { cpu, memory }: &mut Device) -> u8 {
        let (value, new_sp) = memory.stack_pop(cpu.stack_pointer);
        cpu.stack_pointer = new_sp;

        match self.target {
            AccumulatorAndFlags => {
                let [high, low] = value.to_be_bytes();

                cpu.a = high;
                cpu.flags = low;
            }
            Pair(p) => cpu.set(p, value),
        };

        self.cycles().max()
    }
}

impl Execute for Push {
    fn execute(&self, Device { cpu, memory }: &mut Device) -> u8 {
        let value = match self.source {
            AccumulatorAndFlags => u16::from_be_bytes([cpu.a, cpu.flags]),
            Pair(p) => cpu.get(p),
        };

        cpu.stack_pointer = memory.stack_push(cpu.stack_pointer, value);

        self.cycles().max()
    }
}
