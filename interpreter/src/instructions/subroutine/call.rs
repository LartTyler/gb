use crate::Execute;
use gb_asm::{
    instructions::subroutine::call::{Call::*, *},
    Info,
};
use gb_hardware::Device;

impl Execute for Call {
    fn execute(&self, Device { cpu, memory }: &mut Device) -> u8 {
        match self {
            ConstantAddress(cond) => {
                if let Some(cond) = cond {
                    if !cond.test(cpu.flags) {
                        return self.cycles().min();
                    }
                }

                cpu.program_counter = memory.read_word(cpu.program_counter);
            }
            Vector(v) => cpu.program_counter = *v as u16,
        };

        self.cycles().max()
    }
}
