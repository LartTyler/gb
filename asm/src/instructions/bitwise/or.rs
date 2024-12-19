use crate::{
    cycles,
    sources::ByteSource::{self, *},
    Cycles, Info,
};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("OR {source}")]
pub struct Or {
    pub source: ByteSource,
}

impl Info for Or {
    fn bytes(&self) -> u8 {
        match self.source {
            Register(_) | PointerValue => 1,
            ConstantByte => 2,
        }
    }

    fn cycles(&self) -> Cycles {
        match self.source {
            Register(_) => cycles!(1),
            ConstantByte | PointerValue => cycles!(2),
        }
    }
}
