use gb_asm::{sources::ByteSource, Info, Pair};
use gb_hardware::Device;
use gb_parser::{parse, parse_prefixed};

pub mod instructions;
pub mod math;

#[cfg(feature = "inspect")]
pub mod inspector;

#[derive(Debug, Clone, Default)]
pub struct Interpreter {
    #[cfg(feature = "inspect")]
    pub(crate) inspector: inspector::Inspector,
}

impl Interpreter {
    pub fn step(&mut self, device: &mut Device) {
        if let Some(interrupt) = device.get_next_interrupt() {
            if device.is_interrupt_enabled(interrupt) {
                device.stack_push(device.cpu.program_counter);
                device.cpu.interrupts_enabled = false;
                device.cpu.program_counter = interrupt.get_address();

                // According to Pandocs, transitioning to an interrupt handler takes 5 cycles.
                device.cpu.cycle_counter = device.cpu.cycle_counter.wrapping_add(5);
                device.video.process(5 * 4);
            }
        }

        let opcode = device.read_byte(device.cpu.program_counter);
        let instr = parse(opcode).unwrap_or_else(|| {
            panic!(
                "Unimplemented opcode {opcode:#04X} at ${:04X}",
                device.cpu.program_counter
            )
        });

        #[cfg(feature = "inspect")]
        self.inspector.send(inspector::Message::Instruction {
            pc: device.cpu.program_counter,
            instruction: instr,
        });

        device.cpu.program_counter += 1;

        let instr = if instr.is_prefix() {
            let opcode = device.read_byte(device.cpu.program_counter);
            let instr = parse_prefixed(opcode);
            device.cpu.program_counter += 1;

            instr
        } else {
            instr
        };

        let stored_pc = device.cpu.program_counter;

        let cycles = instr.execute(device);
        let cycle_counter = &mut device.cpu.cycle_counter;
        *cycle_counter = cycle_counter.wrapping_add(cycles as u16);

        let pc = &mut device.cpu.program_counter;

        // Some instructions will change the program counter during execution. To ensure we don't
        // end up pointing to the wrong address after an instruction, we need to first check that
        // the PC value we stored before calling `execute()` matches the current value of PC.
        if stored_pc == *pc {
            *pc = pc.wrapping_add((instr.bytes() - 1) as u16);
        }

        device.process(cycles);

        #[cfg(feature = "inspect")]
        self.inspector.send(inspector::Message::Step);
    }
}

pub trait Execute {
    fn execute(&self, device: &mut Device) -> u8;
}

pub trait LoadValue {
    type Value;
    fn load_value(&self, device: &Device) -> Self::Value;
}

impl LoadValue for ByteSource {
    type Value = u8;

    fn load_value(&self, device: &Device) -> Self::Value {
        match self {
            Self::Register(r) => device.cpu.get(r),
            Self::PointerValue => device.read_byte(device.cpu.get(Pair::HL)),
            Self::ConstantByte => device.read_byte(device.cpu.program_counter),
        }
    }
}

pub trait WriteValue {
    type Value;
    fn write_value(&self, device: &mut Device, value: Self::Value);
}
