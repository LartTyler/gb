use super::pop::Source;
use crate::with_simple_info;
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("PUSH {source}")]
pub struct Push {
    pub source: Source,
}

with_simple_info!(Push => (1, 4));
