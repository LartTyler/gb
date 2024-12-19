use crate::{with_simple_info, Pair};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("POP {target}")]
pub struct Pop {
    pub target: Source,
}

with_simple_info!(Pop => (1, 3));

#[derive(Debug, Copy, Clone, Display)]
pub enum Source {
    #[display("AF")]
    AccumulatorAndFlags,
    #[display("r16")]
    Pair(Pair),
}
