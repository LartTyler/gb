use gb_asm::{
    instructions::misc::{
        ComplementAccumulator, ComplementCarryFlag, DecimalAdjustAccumulator, DisableInterrupts,
        EnableInterrupts, Halt,
    },
    Flag, Info,
};
use gb_hardware::Device;

pub fn complement_carry_flag(ccf: &ComplementCarryFlag, Device { cpu, .. }: &mut Device) -> u8 {
    cpu.set(Flag::Subtract, false);
    cpu.set(Flag::HalfCarry, false);
    cpu.set(Flag::Carry, !cpu.get(Flag::Carry));

    ccf.cycles().max()
}

pub fn complement_accumulator(cpl: &ComplementAccumulator, Device { cpu, .. }: &mut Device) -> u8 {
    cpu.a = !cpu.a;

    cpu.set(Flag::Subtract, true);
    cpu.set(Flag::HalfCarry, true);

    cpl.cycles().max()
}

pub fn decimal_adjust_accumulator(
    daa: &DecimalAdjustAccumulator,
    Device { cpu, .. }: &mut Device,
) -> u8 {
    // I'm not even going to pretend I know what the DAA instruction is used for, but [WTF is the
    // DAA instruction](https://ehaskins.com/2018-01-30%20Z80%20DAA/) by Eric Haskins has a great
    // write-up on how to emulate the behavior of the instruction. Code below adapted from their
    // article.

    let mut correction = 0u8;

    if cpu.get(Flag::Carry) || cpu.a > 0x99 {
        correction |= 0x60;
        cpu.set(Flag::Carry, true);
    }

    if cpu.get(Flag::HalfCarry) || (cpu.a & 0xF) > 0x09 {
        correction |= 0x06;
    }

    cpu.a = if cpu.get(Flag::Subtract) {
        cpu.a.wrapping_sub(correction)
    } else {
        cpu.a.wrapping_add(correction)
    };

    cpu.set(Flag::Zero, cpu.a == 0);
    cpu.set(Flag::HalfCarry, false);

    daa.cycles().max()
}

pub fn disable_interrupts(di: &DisableInterrupts, Device { cpu, .. }: &mut Device) -> u8 {
    cpu.interrupts_enabled = false;
    di.cycles().max()
}

pub fn enable_interrupts(ei: &EnableInterrupts, Device { cpu, .. }: &mut Device) -> u8 {
    cpu.interrupts_enabled = true;
    ei.cycles().max()
}

pub fn halt(_halt: &Halt, _device: &mut Device) -> u8 {
    // See https://rgbds.gbdev.io/docs/v0.8.0/gbz80.7#HALT
    todo!()
}
