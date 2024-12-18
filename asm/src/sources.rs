use derive_more::derive::Display;

use crate::Register;

/// Common sources used by many instructions that act on a byte value. Instructions that do not use
/// exactly this set of sources will provide their own source enum.
#[derive(Debug, Copy, Clone, Display)]
pub enum ByteSource {
    #[display("{_0}")]
    Register(Register),
    #[display("(HL)")]
    PointerValue,
    #[display("d8")]
    ConstantByte,
}
