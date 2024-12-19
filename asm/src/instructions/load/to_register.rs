use super::Load;
use crate::{cycles, instructions::Instruction, sources::ByteSource, Cycles, Info, Register};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("{target}, {source}")]
pub struct ToRegister {
    pub target: Register,
    pub source: ByteSource,
}

impl Info for ToRegister {
    fn bytes(&self) -> u8 {
        use ByteSource::*;

        match self.source {
            Register(_) | PointerValue => 1,
            ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        use ByteSource::*;

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
