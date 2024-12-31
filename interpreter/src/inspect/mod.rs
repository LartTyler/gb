use crate::{Interpreter, LoadValue};
use gb_asm::{instructions::Instruction, sources::ByteSource};
use gb_hardware::Device;
use std::sync::mpsc::{self, Receiver, Sender};

pub mod instructions;

#[derive(Debug, Clone, Default)]
pub struct Inspector {
    sender: Option<Sender<Message>>,
}

impl Inspector {
    pub fn connect(&mut self) -> Receiver<Message> {
        let (tx, rx) = mpsc::channel();
        self.sender = Some(tx);

        rx
    }

    pub fn send(&mut self, message: Message) {
        let Some(sender) = &self.sender else {
            return;
        };

        if sender.send(message).is_err() {
            self.sender = None;
        }
    }
}

pub enum Message {
    Instruction { pc: u16, instruction: Instruction },
    Step,
}

impl Interpreter {
    pub fn connect(&mut self) -> Receiver<Message> {
        self.inspector.connect()
    }
}

/*
/// Like `Display`, but intended to give better insight into what an instruction will actually do.
///
/// Instead of executing, the `inspect()` function should return an assembly instruction matching
/// exactly to the encoded instruction (e.g. `LD A, d8` would return `LD A, 72`).
pub trait Inspect {
    fn inspect(&self, device: &Device) -> String;
}

impl Inspect for Instruction {
    fn inspect(&self, device: &Device) -> String {
        match self {
            Self::AddPlusCarry(inner) => inner.inspect(device),
            Self::Add(inner) => inner.inspect(device),
            _ => unimplemented!(),
        }
    }
}

impl Inspect for ByteSource {
    fn inspect(&self, device: &Device) -> String {
        let value = self.load_value(device);
        format!("{self} = {value}")
    }
}
*/
