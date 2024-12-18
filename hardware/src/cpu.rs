use crate::{
    util::{bytes_to_word, word_to_bytes},
    DeviceMode,
};
use gb_asm::{Flag, Pair, Register};

#[derive(Debug, Clone, Default)]
pub struct Cpu {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub flags: u8,
    pub stack_pointer: u16,
    pub program_counter: u16,
    pub cycle_counter: u16,
    pub interrupts_enabled: bool,
}

impl Cpu {
    pub fn new<M>(mode: M) -> Self
    where
        M: Into<DeviceMode>,
    {
        let mut cpu = Self {
            a: 0x11,
            interrupts_enabled: true,
            stack_pointer: 0xFFFE,
            program_counter: 0x100,
            ..Default::default()
        };

        match mode.into() {
            DeviceMode::Classic => cpu.e = 0x08,
            DeviceMode::Color => {
                cpu.d = 0xFF;
                cpu.e = 0x56;
                cpu.l = 0x0D;
            }
        };

        cpu
    }

    pub fn set<T>(&mut self, target: T, value: T::Value)
    where
        T: Settable,
    {
        target.set(self, value)
    }

    pub fn get<T>(&self, target: T) -> T::Value
    where
        T: Gettable,
    {
        target.get(self)
    }
}

pub trait Settable {
    type Value;
    fn set(&self, cpu: &mut Cpu, value: Self::Value);
}

impl Settable for Register {
    type Value = u8;

    fn set(&self, cpu: &mut Cpu, value: Self::Value) {
        use Register::*;

        let target = match self {
            A => &mut cpu.a,
            B => &mut cpu.b,
            C => &mut cpu.c,
            D => &mut cpu.d,
            E => &mut cpu.e,
            H => &mut cpu.h,
            L => &mut cpu.l,
        };

        *target = value;
    }
}

impl Settable for &Register {
    type Value = u8;

    fn set(&self, cpu: &mut Cpu, value: Self::Value) {
        (*self).set(cpu, value)
    }
}

impl Settable for Pair {
    type Value = u16;

    fn set(&self, cpu: &mut Cpu, value: Self::Value) {
        let [high, low] = word_to_bytes(value);
        let [high_reg, low_reg] = self.as_registers();

        cpu.set(high_reg, high);
        cpu.set(low_reg, low);
    }
}

impl Settable for &Pair {
    type Value = u16;

    fn set(&self, cpu: &mut Cpu, value: Self::Value) {
        (*self).set(cpu, value)
    }
}

impl Settable for Flag {
    type Value = bool;

    fn set(&self, cpu: &mut Cpu, value: Self::Value) {
        let bit = *self as u8;

        if value {
            cpu.flags |= bit;
        } else {
            cpu.flags &= !bit;
        }
    }
}

pub trait Gettable {
    type Value;
    fn get(&self, cpu: &Cpu) -> Self::Value;
}

impl Gettable for Register {
    type Value = u8;

    fn get(&self, cpu: &Cpu) -> Self::Value {
        match self {
            Self::A => cpu.a,
            Self::B => cpu.b,
            Self::C => cpu.c,
            Self::D => cpu.d,
            Self::E => cpu.e,
            Self::H => cpu.h,
            Self::L => cpu.l,
        }
    }
}

impl Gettable for &Register {
    type Value = u8;

    fn get(&self, cpu: &Cpu) -> Self::Value {
        (*self).get(cpu)
    }
}

impl Gettable for Pair {
    type Value = u16;

    fn get(&self, cpu: &Cpu) -> Self::Value {
        let [high_reg, low_reg] = self.as_registers();
        bytes_to_word(cpu.get(high_reg), cpu.get(low_reg))
    }
}

impl Gettable for &Pair {
    type Value = u16;

    fn get(&self, cpu: &Cpu) -> Self::Value {
        (*self).get(cpu)
    }
}

impl Gettable for Flag {
    type Value = bool;

    fn get(&self, cpu: &Cpu) -> Self::Value {
        cpu.flags & (*self as u8) != 0
    }
}

impl Gettable for &Flag {
    type Value = bool;

    fn get(&self, cpu: &Cpu) -> Self::Value {
        (*self).get(cpu)
    }
}
