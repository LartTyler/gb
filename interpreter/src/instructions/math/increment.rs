use crate::{math::GbAdd, Execute};
use gb_asm::{
    instructions::math::{dec::Target::*, inc::*},
    Flag, Info, Pair,
};
use gb_hardware::Device;

impl Execute for Increment {
    fn execute(&self, Device { cpu, memory }: &mut Device) -> u8 {
        match self.target {
            Register(r) => {
                let result = cpu.get(r).add(1);
                cpu.set(r, result.value);

                cpu.set(Flag::Zero, result.value == 0);
                cpu.set(Flag::Subtract, false);
                cpu.set(Flag::HalfCarry, result.half_carry);
            }
            PointerValue => {
                let address = cpu.get(Pair::HL);
                let result = memory.read_byte(address).add(1);
                memory.write_byte(address, result.value);

                cpu.set(Flag::Zero, result.value == 0);
                cpu.set(Flag::Subtract, false);
                cpu.set(Flag::HalfCarry, result.half_carry);
            }
            Pair(p) => {
                let result = cpu.get(p).add(1);
                cpu.set(p, result.value);
            }
            StackPointer => {
                let result = cpu.stack_pointer.add(1);
                cpu.stack_pointer = result.value;
            }
        };

        self.cycles().max()
    }
}
