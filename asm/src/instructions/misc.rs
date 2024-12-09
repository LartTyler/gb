use crate::{impl_info, Cycles, Info};
use derive_more::derive::Display;

#[derive(Debug, Copy, Clone, Display)]
#[display("CCF")]
pub struct ComplementCarryFlag;
impl_info!(ComplementCarryFlag => (1, 1));

#[derive(Debug, Copy, Clone, Display)]
#[display("CPL")]
pub struct ComplementAccumulator;
impl_info!(ComplementAccumulator => (1, 1));

#[derive(Debug, Copy, Clone, Display)]
#[display("DAA")]
pub struct DecimalAdjustAccumulator;
impl_info!(DecimalAdjustAccumulator => (1, 1));

#[derive(Debug, Copy, Clone, Display)]
#[display("DI")]
pub struct DisableInterrupts;
impl_info!(DisableInterrupts => (1, 1));

#[derive(Debug, Copy, Clone, Display)]
#[display("EI")]
pub struct EnableInterrupts;
impl_info!(EnableInterrupts => (1, 1));

#[derive(Debug, Copy, Clone, Display)]
#[display("HALT")]
pub struct Halt;
impl_info!(Halt => (0, 1));
