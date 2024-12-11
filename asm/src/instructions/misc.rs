use crate::with_simple_info;
use derive_more::derive::Display;

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("CCF")]
    pub struct ComplementCarryFlag => (1, 1);
}

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("CPL")]
    pub struct ComplementAccumulator => (1, 1);
}

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("DAA")]
    pub struct DecimalAdjustAccumulator => (1, 1);
}

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("DI")]
    pub struct DisableInterrupts => (1, 1);
}

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("EI")]
    pub struct EnableInterrupts => (1, 1);
}

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("HALT")]
    pub struct Halt => (0, 1);
}

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("NOP")]
    pub struct Nop => (1, 1);
}

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("SCF")]
    pub struct SetCarryFlag => (1, 1);
}

with_simple_info! {
    #[derive(Debug, Copy, Clone, Display)]
    #[display("STOP")]
    pub struct Stop => (0, 2);
}
