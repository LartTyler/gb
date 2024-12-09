use std::fmt::Display;

pub mod instructions;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum Flag {
    Carry = 0b0001_0000,
    HalfCarry = 0b0010_0000,
    Subtract = 0b0100_0000,
    Zero = 0b1000_0000,
}

#[derive(Debug, Copy, Clone)]
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
    ( $base:expr $(, $max:expr )? ) => {
        $crate::Cycles::from($base $(, $max)? )
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
                $( #[doc = $doc:expr] )*
                $variant:ident ($inner:path) $(,)?
            ),*
        }
    ) => {
        $( #[$meta] )*
        $vis enum $type {
            $(
                $( #[doc = $doc] )*
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
