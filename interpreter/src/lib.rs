use gb_asm::{sources::ByteSource, Info, Pair};
use gb_hardware::Device;
use gb_parser::{parse, parse_prefixed};

pub mod instructions;
pub mod math;

#[cfg(feature = "inspect")]
pub mod inspect;

#[derive(Debug, Clone, Default)]
pub struct Interpreter {
    #[cfg(feature = "inspect")]
    pub(crate) inspector: inspect::Inspector,
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

        let base_pc = device.cpu.program_counter;
        let opcode = device.read_byte(base_pc);
        let instr = parse(opcode).unwrap_or_else(|| {
            panic!(
                "Unimplemented opcode {opcode:#04X} at ${:04X}",
                device.cpu.program_counter
            )
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

        #[cfg(feature = "inspect")]
        self.inspector.send(inspect::Message::Instruction {
            pc: base_pc,
            instruction: instr,
        });

        // We store the mid-process value of PC before executing the instruction so we can detect
        // changes to the PC by an instruction. Some instructions (such as jumps or calls) can
        // modify PC, and if they do, we don't want to change PC again after they're done
        // executing.
        let pre_exec_pc = device.cpu.program_counter;

        let cycles = instr.execute(device);
        let cycle_counter = &mut device.cpu.cycle_counter;
        *cycle_counter = cycle_counter.wrapping_add(cycles as u16);

        let pc = &mut device.cpu.program_counter;

        // As long as PC didn't change during instruction execution, we're safe to move to the
        // next instruction.
        if pre_exec_pc == *pc {
            *pc = base_pc.wrapping_add(instr.bytes() as u16);
        }

        device.process(cycles);

        #[cfg(feature = "inspect")]
        self.inspector.send(inspect::Message::Step);
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
