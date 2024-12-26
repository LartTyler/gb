use crate::Execute;
use gb_asm::{instructions::subroutine::ret::*, Info};
use gb_hardware::Device;

impl Execute for Return {
    fn execute(&self, device: &mut Device) -> u8 {
        if let Self::Normal(Some(cond)) = self {
            if !cond.test(device.cpu.flags) {
                return self.cycles().min();
            }
        }

        let new_pc = device.stack_pop();
        device.cpu.program_counter = new_pc;

        if matches!(self, Self::EnableInterrupts) {
            device.cpu.interrupts_enabled = true;
        }

        self.cycles().max()
    }
}
