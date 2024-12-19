use crate::{
    cycles,
    sources::ByteSource::{self, *},
    Cycles, Info,
};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("XOR {source}")]
pub struct Xor {
    pub source: ByteSource,
}

impl Info for Xor {
    fn bytes(&self) -> u8 {
        match self.source {
            Register(_) | PointerValue => 1,
            ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        match self.source {
            Register(_) => cycles!(1),
            PointerValue | ConstantByte => cycles!(2),
        }
    }
}
