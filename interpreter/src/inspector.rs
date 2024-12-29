use crate::Interpreter;
use gb_asm::instructions::Instruction;
use std::sync::mpsc::{self, Receiver, Sender};

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
