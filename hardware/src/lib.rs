use cpu::Cpu;
use memory::{map::*, Memory, MemoryError};
use std::{collections::HashSet, fs::File, io::Read, path::Path};
use video::{Video, REGISTER_LCD_STATUS, REGISTER_LCD_Y_COMPARE, REGISTER_LCD_Y_COORD};

pub mod cpu;
pub mod memory;
pub mod util;
pub mod video;

#[derive(Debug, Copy, Clone)]
pub enum DeviceMode {
    Classic,
    Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Interrupt {
    VerticalBlank,
    Stat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    pub fn get_mask(&self) -> u8 {
        match self {
            Self::VerticalBlank => 0b0000_0001,
            Self::Stat => 0b0000_0010,
            Self::Timer => 0b0000_0100,
            Self::Serial => 0b0000_1000,
            Self::Joypad => 0b0001_0000,
        }
    }

    pub fn get_address(&self) -> u16 {
        match self {
            Self::VerticalBlank => 0x40,
            Self::Stat => 0x48,
            Self::Timer => 0x50,
            Self::Serial => 0x58,
            Self::Joypad => 0x60,
        }
    }
}

pub struct Device {
    pub cpu: Cpu,
    pub memory: Memory,
    pub video: Video,
    pub interrupts_pending: HashSet<Interrupt>,
    previous_stat_value: bool,
}

impl Device {
    pub fn from_file(cart_file: &Path) -> Result<Self, Error> {
        let mut file = File::open(cart_file)?;
        let len = file.metadata()?.len();
        let len: usize = len.try_into().map_err(|_| Error::FileTooBig)?;

        let mut rom: Vec<u8> = Vec::with_capacity(len);
        file.read_to_end(&mut rom)?;

        let memory = Memory::new(rom)?;
        let device_mode = DeviceMode::from(memory.cartridge.device_mode);

        Ok(Self {
            cpu: Cpu::new(device_mode),
            video: Video::new(device_mode),
            interrupts_pending: HashSet::new(),
            previous_stat_value: false,
            memory,
        })
    }

    pub fn is_interrupt_enabled(&self, interrupt: Interrupt) -> bool {
        self.cpu.interrupts_enabled && (self.memory.interrupts_enabled & interrupt.get_mask() > 0)
    }

    pub fn process(&mut self, delta: u8) {
        // PPU cycles (also referred to as "dots") are actually t-cycles (m_cycle * 4)
        self.video.process(delta * 4);

        if self.video.has_vblank_interrupt {
            self.interrupts_pending.insert(Interrupt::VerticalBlank);
        } else {
            self.interrupts_pending.remove(&Interrupt::VerticalBlank);
        }

        // STAT interrupts only trigger on the rising edge, meaning that two sequential STAT
        // interrupts (e.g. when going from VBlank to HBlank) do not actually trigger a STAT
        // interrupt. To emulate this behavior, we store the last-known value of the STAT flag from
        // the video device, and only queue a STAT interrupt if we've gone from `false` (low
        // signal) to `true` (high signal).
        if self.video.has_stat_interrupt && !self.previous_stat_value {
            self.interrupts_pending.insert(Interrupt::Stat);
        } else {
            self.interrupts_pending.remove(&Interrupt::Stat);
        }

        self.previous_stat_value = self.video.has_stat_interrupt;
    }

    pub fn get_next_interrupt(&mut self) -> Option<Interrupt> {
        let int = self.interrupts_pending.take(&Interrupt::VerticalBlank);

        if int.is_some() {
            return int;
        }

        let int = self.interrupts_pending.take(&Interrupt::Stat);

        if int.is_some() {
            return int;
        }

        let int = self.interrupts_pending.take(&Interrupt::Timer);

        if int.is_some() {
            return int;
        }

        let int = self.interrupts_pending.take(&Interrupt::Joypad);

        if int.is_some() {
            return int;
        }

        None
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;

        let slot = match address {
            ROM0_START..=ROM0_END | ROM_BANK_START..=ROM_BANK_END => {
                return self.memory.cartridge.rom_read(address)
            }
            VRAM_START..=VRAM_END => self.video.vram.get(address - VRAM_START),
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => {
                return self.memory.cartridge.ram_read(address)
            }
            RAM0_START..=RAM0_END => self.memory.wram.get_from(0, address - RAM0_START),
            RAM_BANK_START..=RAM_BANK_END => self.memory.wram.get(address - RAM_BANK_START),
            ECHO_START..=ECHO_END => self.memory.wram.get(address - ECHO_START),
            OAM_START..=OAM_END => self.memory.oam.get(address - OAM_START),
            UNUSED_START..=UNUSED_END => Some(&0),
            INTERRUPT_FLAGS => Some(&self.memory.interrupt_flags),
            REGISTER_LCD_STATUS => Some(&self.video.status_register),
            REGISTER_LCD_Y_COORD => Some(&self.video.current_line),
            REGISTER_LCD_Y_COMPARE => Some(&self.video.current_line_compare),
            IO_START..=IO_END => self.memory.io.get(address - IO_START),
            HRAM_START..=HRAM_END => self.memory.hram.get(address - HRAM_START),
            INTERRUPT_ENABLED => Some(&self.memory.interrupts_enabled),
            _ => unreachable!(),
        };

        *slot.unwrap_or(&0xFF)
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let high = self.read_byte(address);
        let low = self.read_byte(address.wrapping_add(1));

        u16::from_be_bytes([high, low])
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;

        let slot = match address {
            ROM0_START..=ROM0_END | ROM_BANK_START..=ROM_BANK_END => {
                self.memory.cartridge.rom_write(address, value);
                return;
            }
            VRAM_START..=VRAM_END => self.video.vram.get_mut(address - VRAM_START),
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => {
                self.memory.cartridge.ram_write(address, value);
                return;
            }
            RAM0_START..=RAM0_END => self.memory.wram.get_from_mut(0, address - RAM0_START),
            RAM_BANK_START..=RAM_BANK_END => self.memory.wram.get_mut(address - RAM_BANK_START),
            ECHO_START..=ECHO_END => self.memory.wram.get_mut(address - ECHO_START),
            OAM_START..=OAM_END => self.memory.oam.get_mut(address - OAM_START),
            UNUSED_START..=UNUSED_END => return,
            INTERRUPT_FLAGS => Some(&mut self.memory.interrupt_flags),
            REGISTER_LCD_STATUS => {
                self.video.write_status_register(value);
                return;
            }
            REGISTER_LCD_Y_COORD => return,
            REGISTER_LCD_Y_COMPARE => Some(&mut self.video.current_line_compare),
            IO_START..=IO_END => self.memory.io.get_mut(address - IO_START),
            HRAM_START..=HRAM_END => self.memory.hram.get_mut(address - HRAM_START),
            INTERRUPT_ENABLED => Some(&mut self.memory.interrupts_enabled),
            _ => unreachable!(),
        };

        if let Some(slot) = slot {
            *slot = value;
        }
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let [high, low] = value.to_be_bytes();

        self.write_byte(address, low);
        self.write_byte(address.wrapping_add(1), high)
    }

    pub fn stack_push(&mut self, value: u16) {
        let [high, low] = value.to_be_bytes();
        let stack_pointer = self.cpu.stack_pointer;

        self.write_byte(stack_pointer, high);
        let stack_pointer = stack_pointer.wrapping_sub(1);

        self.write_byte(stack_pointer, low);
        self.cpu.stack_pointer = stack_pointer.wrapping_sub(1);
    }

    pub fn stack_pop(&mut self) -> u16 {
        let stack_pointer = self.cpu.stack_pointer;

        let low = self.read_byte(stack_pointer);
        let stack_pointer = stack_pointer.wrapping_add(1);

        let high = self.read_byte(stack_pointer);
        self.cpu.stack_pointer = stack_pointer.wrapping_add(1);

        u16::from_be_bytes([high, low])
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("memory error: {0}")]
    Memory(#[from] MemoryError),
    #[error("cart file size too big")]
    FileTooBig,
}
