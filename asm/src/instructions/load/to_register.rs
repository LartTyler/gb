use crate::{cycles, instructions::Instruction, Cycles, Info, Register};
use derive_more::derive::Display;

use super::Load;

#[derive(Debug, Copy, Clone, Display)]
#[display("{target}, {source}")]
pub struct ToRegister {
    pub target: Register,
    pub source: ToRegisterSource,
}

impl Info for ToRegister {
    fn bytes(&self) -> u8 {
        use ToRegisterSource::*;

        match self.source {
            Register(_) | PointerValue => 1,
            ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        use ToRegisterSource::*;

        match self.source {
            Register(_) => cycles!(1),
            PointerValue | ConstantByte => cycles!(2),
        }
    }
}

impl From<ToRegister> for Instruction {
    fn from(value: ToRegister) -> Self {
        Load::from(value).into()
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum ToRegisterSource {
    #[display("{_0}")]
    Register(Register),
    #[display("d8")]
    ConstantByte,
    #[display("(HL)")]
    PointerValue,
}
