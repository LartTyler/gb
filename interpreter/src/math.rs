use gb_asm::Flag;
use gb_hardware::cpu::Cpu;

#[derive(Debug, Clone, Default)]
pub struct MathResult<Value> {
    pub half_carry: bool,
    pub carry: bool,
    pub value: Value,
}

impl MathResult<u8> {
    pub fn copy_to_cpu_flags(&self, cpu: &mut Cpu) {
        cpu.set(Flag::Zero, self.value.is_zero());
        cpu.set(Flag::Carry, self.carry);
        cpu.set(Flag::HalfCarry, self.half_carry);
    }
}

impl MathResult<u16> {
    pub fn copy_to_cpu_flags(&self, cpu: &mut Cpu) {
        cpu.set(Flag::Carry, self.carry);
        cpu.set(Flag::HalfCarry, self.half_carry);
    }
}

impl<T> MathResult<T>
where
    T: Default,
{
    pub fn merge(self, other: Self) -> Self {
        Self {
            half_carry: self.half_carry || other.half_carry,
            carry: self.carry || other.carry,
            value: other.value,
        }
    }
}

pub trait GbAdd<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> MathResult<Self::Output>;
    fn add_with_carry(self, rhs: Rhs, carry: bool) -> MathResult<Self::Output>;
}

impl<T> GbAdd for T
where
    T: Operand + Copy + Default,
{
    type Output = Self;

    fn add(self, rhs: Self) -> MathResult<Self::Output> {
        let half_carry = self.is_half_carry_add(rhs);
        let (result, carry) = self.overflowing_add(rhs);

        MathResult {
            half_carry,
            carry,
            value: result,
        }
    }

    fn add_with_carry(self, rhs: Self, carry: bool) -> MathResult<Self::Output> {
        let result = carry
            .then(|| self.add(Self::get_carry_value()))
            .unwrap_or_default();

        result.merge(self.add(rhs))
    }
}

pub trait GbSub<Rhs = Self> {
    type Output;

    fn sub(self, rhs: Rhs) -> MathResult<Self::Output>;
    fn sub_with_carry(self, rhs: Rhs, carry: bool) -> MathResult<Self::Output>;
}

impl<T> GbSub for T
where
    T: Operand + Copy + Default,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> MathResult<Self::Output> {
        let half_carry = self.is_half_carry_sub(rhs);
        let (result, carry) = self.overflowing_sub(rhs);

        MathResult {
            half_carry,
            carry,
            value: result,
        }
    }

    fn sub_with_carry(self, rhs: Self, carry: bool) -> MathResult<Self::Output> {
        let result = carry
            .then(|| self.sub(Self::get_carry_value()))
            .unwrap_or_default();

        result.merge(self.sub(rhs))
    }
}

pub trait Operand<Rhs = Self>
where
    Self: Sized,
{
    fn overflowing_add(self, rhs: Rhs) -> (Self, bool);
    fn overflowing_sub(self, rhs: Rhs) -> (Self, bool);
    fn carrying_shl(self, rhs: u32, carry: bool) -> (Self, bool);
    fn carrying_shr(self, rhs: u32, carry: bool) -> (Self, bool);
    fn is_half_carry_add(&self, rhs: Rhs) -> bool;
    fn is_half_carry_sub(&self, rhs: Rhs) -> bool;
    fn is_zero(&self) -> bool;
    fn get_carry_value() -> Self;
    fn get_bits() -> Self;
}

macro_rules! carrying_fns {
    () => {
        fn carrying_shl(self, rhs: u32, carry: bool) -> (Self, bool) {
            let (mut result, new_carry) = self.overflowing_shl(rhs);

            if carry {
                result |= 1;
            }

            (result, new_carry)
        }

        fn carrying_shr(self, rhs: u32, carry: bool) -> (Self, bool) {
            let (mut result, new_carry) = self.overflowing_shr(rhs);

            if carry {
                result |= 1 << (Self::BITS - 1);
            }

            (result, new_carry)
        }
    };
}

impl Operand for u8 {
    carrying_fns!();

    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        Self::overflowing_add(self, rhs)
    }

    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        Self::overflowing_sub(self, rhs)
    }

    fn is_half_carry_add(&self, rhs: Self) -> bool {
        let lhs = self & 0xF;
        let rhs = rhs & 0xF;

        lhs + rhs > 0xF
    }

    fn is_half_carry_sub(&self, rhs: Self) -> bool {
        (self & 0xF) > (rhs & 0xF)
    }

    fn get_carry_value() -> Self {
        1
    }

    fn get_bits() -> Self {
        Self::BITS as Self
    }

    fn is_zero(&self) -> bool {
        self == &0
    }
}

impl Operand for u16 {
    carrying_fns!();

    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        Self::overflowing_add(self, rhs)
    }

    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        Self::overflowing_sub(self, rhs)
    }

    fn is_half_carry_add(&self, rhs: Self) -> bool {
        // There's a lot of conflicting information out there about how the Gameboy actually does
        // math. For my implementation, I'm going off the assertion that the ALU operates in 4-bit
        // steps (thus the need for a half-carry flag in the first place).
        //
        // Since a 16-bit operation would then, naturally, require 4 4-bit operations, in order to
        // determine a half-carry for a 16-bit operation we would need to examine the third 4-bit
        // operation. To emulate this, we discard the bottom 8-bits (the first two 4-bit
        // operators), and then apply our half-carry logic to the now-lowest 4-bits of the value.
        let lhs = (self >> 8) & 0xF;
        let rhs = (rhs >> 8) & 0xF;

        lhs + rhs > 0xF
    }

    fn is_half_carry_sub(&self, rhs: Self) -> bool {
        let lhs = (self >> 8) & 0xF;
        let rhs = (rhs >> 8) & 0xF;

        lhs > rhs
    }

    fn get_carry_value() -> Self {
        1
    }

    fn get_bits() -> Self {
        Self::BITS as Self
    }

    fn is_zero(&self) -> bool {
        self == &0
    }
}
