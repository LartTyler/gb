use crate::Execute;
use gb_asm::{
    instructions::subroutine::call::{Call::*, *},
    Info,
};
use gb_hardware::Device;

impl Execute for Call {
    fn execute(&self, Device { cpu, memory }: &mut Device) -> u8 {
        // The next PC position is one less than the width of this instruction, since PC is
        // positioned after the opcode when executing instructions.
        let next_pc = cpu.program_counter.wrapping_add(self.bytes() as u16 - 1);
        cpu.stack_pointer = memory.stack_push(cpu.stack_pointer, next_pc);

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
