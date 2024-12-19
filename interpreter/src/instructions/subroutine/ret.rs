use crate::Execute;
use gb_asm::{instructions::subroutine::ret::*, Info};
use gb_hardware::Device;

impl Execute for Return {
    fn execute(&self, Device { cpu, memory }: &mut Device) -> u8 {
        if let Self::Normal(Some(cond)) = self {
            if !cond.test(cpu.flags) {
                return self.cycles().min();
            }
        }

        let (new_pc, new_sp) = memory.stack_pop(cpu.stack_pointer);
        cpu.stack_pointer = new_sp;
        cpu.program_counter = new_pc;

        if matches!(self, Self::EnableInterrupts) {
            cpu.interrupts_enabled = true;
        }

        self.cycles().max()
    }
}
