use crate::{math::GbAdd, Execute};
use gb_asm::{
    instructions::math::{dec::Target::*, inc::*},
    Flag, Info, Pair,
};
use gb_hardware::Device;

impl Execute for Increment {
    fn execute(&self, device: &mut Device) -> u8 {
        match self.target {
            Register(r) => {
                let result = device.cpu.get(r).add(1);
                device.cpu.set(r, result.value);

                device.cpu.set(Flag::Zero, result.value == 0);
                device.cpu.set(Flag::Subtract, false);
                device.cpu.set(Flag::HalfCarry, result.half_carry);
            }
            PointerValue => {
                let address = device.cpu.get(Pair::HL);
                let result = device.read_byte(address).add(1);
                device.write_byte(address, result.value);

                device.cpu.set(Flag::Zero, result.value == 0);
                device.cpu.set(Flag::Subtract, false);
                device.cpu.set(Flag::HalfCarry, result.half_carry);
            }
            Pair(p) => {
                let result = device.cpu.get(p).add(1);
                device.cpu.set(p, result.value);
            }
            StackPointer => {
                let result = device.cpu.stack_pointer.add(1);
                device.cpu.stack_pointer = result.value;
            }
        };

        self.cycles().max()
    }
}
