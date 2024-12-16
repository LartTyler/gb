use gb_asm::instructions::{misc::*, Instruction};

macro_rules! create_fn {
    ($name:ident => $inner:path) => {
        pub fn $name() -> Instruction {
            $inner.into()
        }
    };
}

create_fn!(complement_carry_flag => ComplementCarryFlag);
create_fn!(complement_accumulator => ComplementAccumulator);
create_fn!(decimal_adjust_acumulator => DecimalAdjustAccumulator);
create_fn!(disable_interrupts => DisableInterrupts);
create_fn!(enable_interrupts => EnableInterrupts);
create_fn!(halt => Halt);
create_fn!(nop => Nop);
create_fn!(set_carry_flag => SetCarryFlag);
create_fn!(stop => Stop);
create_fn!(prefix => Prefix);
