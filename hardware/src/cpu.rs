use crate::util::{bytes_to_word, word_to_bytes};
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
}

impl Cpu {
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

    pub fn set_flag(&mut self, flag: Flag, on: bool) {
        if on {
            self.flags |= flag as u8;
        } else {
            self.flags &= !(flag as u8);
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        self.flags & (flag as u8) != 0
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

impl Settable for Pair {
    type Value = u16;

    fn set(&self, cpu: &mut Cpu, value: Self::Value) {
        let [high, low] = word_to_bytes(value);
        let [high_reg, low_reg] = self.as_registers();

        cpu.set(high_reg, high);
        cpu.set(low_reg, low);
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

impl Gettable for Pair {
    type Value = u16;

    fn get(&self, cpu: &Cpu) -> Self::Value {
        let [high_reg, low_reg] = self.as_registers();
        bytes_to_word(cpu.get(high_reg), cpu.get(low_reg))
    }
}
