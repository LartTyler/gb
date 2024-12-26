use crate::Execute;
use gb_asm::{
    instructions::subroutine::call::{Call::*, *},
    Info,
};
use gb_hardware::Device;

impl Execute for Call {
    fn execute(&self, device: &mut Device) -> u8 {
        // The next PC position is one less than the width of this instruction, since PC is
        // positioned after the opcode when executing instructions.
        let next_pc = device
            .cpu
            .program_counter
            .wrapping_add(self.bytes() as u16 - 1);

        device.stack_push(next_pc);

        match self {
            ConstantAddress(cond) => {
                if let Some(cond) = cond {
                    if !cond.test(device.cpu.flags) {
                        return self.cycles().min();
                    }
                }

                device.cpu.program_counter = device.read_word(device.cpu.program_counter);
            }
            Vector(v) => device.cpu.program_counter = *v as u16,
        };

        self.cycles().max()
    }
}
