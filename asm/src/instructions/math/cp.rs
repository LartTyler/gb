use crate::{
    cycles,
    sources::ByteSource::{self, *},
    Cycles, Info,
};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("CP {source}")]
pub struct Compare {
    pub source: ByteSource,
}

impl Info for Compare {
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
