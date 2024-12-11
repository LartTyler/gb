use crate::{with_simple_info, Pair};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
pub enum Pop {
    #[display("POP AF")]
    AccumulatorAndFlags,
    #[display("POP r16")]
    Pair(Pair),
}

with_simple_info!(Pop => (1, 3));
