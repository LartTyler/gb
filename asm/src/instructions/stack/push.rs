use crate::{with_simple_info, Pair};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
pub enum Push {
    #[display("PUSH AF")]
    AccumulatorAndFlags,
    #[display("PUSH r16")]
    Pair(Pair),
}

with_simple_info!(Push => (1, 4));
