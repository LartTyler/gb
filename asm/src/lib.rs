use derive_more::derive::Display;
use std::fmt::Display;

pub mod instructions;
pub mod sources;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
            Self::H => 'H',
            Self::L => 'L',
        };

        write!(f, "{symbol}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Pair {
    BC,
    DE,
    HL,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::BC => "BC",
            Self::DE => "DE",
            Self::HL => "HL",
        };

        write!(f, "{symbol}")
    }
}

impl Pair {
    pub fn as_registers(&self) -> [Register; 2] {
        match self {
            Self::BC => [Register::B, Register::C],
            Self::DE => [Register::D, Register::E],
            Self::HL => [Register::H, Register::L],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Flag {
    Carry = 0b0001_0000,
    HalfCarry = 0b0010_0000,
    Subtract = 0b0100_0000,
    Zero = 0b1000_0000,
}

impl Flag {
    #[inline]
    fn mask(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cycles {
    Variable(u8, u8),
    Fixed(u8),
}

impl Cycles {
    pub fn min(&self) -> u8 {
        match self {
            Self::Fixed(v) => *v,
            Self::Variable(v, _) => *v,
        }
    }

    pub fn max(&self) -> u8 {
        match self {
            Self::Fixed(v) => *v,
            Self::Variable(_, v) => *v,
        }
    }
}

impl From<u8> for Cycles {
    fn from(value: u8) -> Self {
        Cycles::Fixed(value)
    }
}

impl From<(u8, u8)> for Cycles {
    fn from(value: (u8, u8)) -> Self {
        Cycles::Variable(value.0, value.1)
    }
}

#[macro_export]
macro_rules! cycles {
    ( $base:expr ) => {
        $crate::Cycles::from($base)
    };

    ( $base:expr, $max:expr ) => {
        $crate::Cycles::from(($base, $max))
    };
}

pub trait Info {
    fn bytes(&self) -> u8;
    fn cycles(&self) -> Cycles;
}

#[macro_export]
macro_rules! with_info_trait {
    (
        $( #[$meta:meta] )*
        $vis:vis enum $type:ident {
            $(
                $( #[$variant_meta:meta] )*
                $variant:ident ($inner:path) $(,)?
            ),*
        }
    ) => {
        $( #[$meta] )*
        $vis enum $type {
            $(
                $( #[$variant_meta] )*
                $variant ($inner)
            ),*
        }

        impl $crate::Info for $type {
            fn bytes(&self) -> u8 {
                match self {
                    $( Self::$variant(v) => v.bytes() ),*
                }
            }

            fn cycles(&self) -> Cycles {
                match self {
                    $( Self::$variant(v) => v.cycles() ),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! with_simple_info {
    (
        $( #[$meta:meta] )*
        $vis:vis struct $type:ident => ( $bytes:expr, $cycles:expr );
    ) => {
        $( #[$meta] )*
        $vis struct $type;

        impl $crate::Info for $type {
            fn bytes(&self) -> u8 {
                $bytes
            }

            fn cycles(&self) -> $crate::Cycles {
                $cycles.into()
            }
        }
    };

    ( $type:ty => ($bytes:expr, $cycles:expr) ) => {
        impl $crate::Info for $type {
            fn bytes(&self) -> u8 {
                $bytes
            }

            fn cycles(&self) -> $crate::Cycles {
                $cycles.into()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_info {
    ( $type:ty => ( $bytes:literal, $cycles:expr ) ) => {
        impl Info for $type {
            fn bytes(&self) -> u8 {
                $bytes
            }

            fn cycles(&self) -> Cycles {
                $cycles.into()
            }
        }
    };
}

#[derive(Debug, Copy, Clone, Display, PartialEq, Eq)]
#[display("{_0}")]
pub struct Bit(u8);

#[macro_export]
macro_rules! constant_value_fns {
    (
        $(
            $name:ident => $value:expr $(,)?
        ),+
    ) => {
        $(
            pub fn $name() -> Self {
                Self($value)
            }
        )*
    };
}

impl Bit {
    const MAX_VALUE: u8 = 7;

    pub fn new(value: u8) -> Result<Self, &'static str> {
        if value > Self::MAX_VALUE {
            Err("bit index out of range, must be between zero and seven (inclusive)")
        } else {
            Ok(Self(value))
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    constant_value_fns! {
        zero => 0,
        one => 1,
        two => 2,
        three => 3,
        four => 4,
        five => 5,
        six => 6,
        seven => 7,
    }
}
