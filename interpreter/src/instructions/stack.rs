use crate::Execute;
use gb_asm::{
    instructions::stack::{
        pop::{Source::*, *},
        push::*,
    },
    Info,
};
use gb_hardware::{
    util::{bytes_to_word, word_to_bytes},
    Device,
};

impl Execute for Pop {
    fn execute(&self, device: &mut Device) -> u8 {
        let value = device.stack_pop();

        match self.target {
            AccumulatorAndFlags => {
                let [low, high] = word_to_bytes(value);

                device.cpu.a = high;
                device.cpu.flags = low;
            }
            Pair(p) => device.cpu.set(p, value),
        };

        self.cycles().max()
    }
}

impl Execute for Push {
    fn execute(&self, device: &mut Device) -> u8 {
        let value = match self.source {
            AccumulatorAndFlags => bytes_to_word(device.cpu.a, device.cpu.flags),
            Pair(p) => device.cpu.get(p),
        };

        device.stack_push(value);

        self.cycles().max()
    }
}
