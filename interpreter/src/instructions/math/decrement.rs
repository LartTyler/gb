use crate::{math::GbSub, Execute};
use gb_asm::{
    instructions::math::dec::{Target::*, *},
    Flag, Info, Pair,
};
use gb_hardware::Device;

impl Execute for Decrement {
    fn execute(&self, Device { cpu, memory }: &mut Device) -> u8 {
        match self.target {
            Register(r) => {
                let result = cpu.get(r).sub(1);
                cpu.set(r, result.value);

                cpu.set(Flag::Subtract, true);
                cpu.set(Flag::Zero, result.value == 0);
                cpu.set(Flag::HalfCarry, result.half_carry);
            }
            PointerValue => {
                let address = cpu.get(Pair::HL);
                let result = memory.read_byte(address).sub(1);
                memory.write_byte(address, result.value);

                cpu.set(Flag::Subtract, true);
                cpu.set(Flag::Zero, result.value == 0);
                cpu.set(Flag::HalfCarry, result.half_carry);
            }
            Pair(p) => {
                let result = cpu.get(p).sub(1);
                cpu.set(p, result.value);
            }
            StackPointer => {
                let result = cpu.stack_pointer.sub(1);
                cpu.stack_pointer = result.value;
            }
        };

        self.cycles().max()
    }
}
