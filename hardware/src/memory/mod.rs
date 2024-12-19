use crate::DeviceMode;
use cartridge::{Cartridge, CartridgeError};
use map::*;

pub mod cartridge;
pub mod map;

pub struct Memory {
    pub cartridge: Cartridge,
    vram: Vec<u8>,
    wram: Vec<u8>,
    oam: Vec<u8>,
    io: Vec<u8>,
    hram: Vec<u8>,
    interrupt_flags: u8,
    interrupt_enable: u8,
    vram_bank: usize,
    wram_bank: usize,
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> Result<Self, MemoryError> {
        let cartridge = Cartridge::new(rom)?;

        let (vram, wram) = match cartridge.device_mode.into() {
            DeviceMode::Color => (vec![0; VRAM_SIZE * 2], vec![0; RAM_BANK_SIZE * 8]),
            DeviceMode::Classic => (vec![0; VRAM_SIZE], vec![0; RAM_BANK_SIZE * 2]),
        };

        Ok(Self {
            cartridge,
            vram,
            wram,
            oam: vec![0; OAM_SIZE],
            io: vec![0; IO_SIZE],
            hram: vec![0; HRAM_SIZE],
            interrupt_flags: 0,
            interrupt_enable: 0,
            vram_bank: 0,
            wram_bank: 1,
        })
    }

    pub fn read_byte<A>(&self, address: A) -> u8
    where
        A: Into<usize>,
    {
        let address = address.into();

        let slot = match address {
            ROM0_START..=ROM0_END | ROM_BANK_START..=ROM_BANK_END => {
                return self.cartridge.rom_read(address)
            }
            VRAM_START..=VRAM_END => {
                let address = self.vram_bank * VRAM_SIZE + (address - VRAM_START);
                self.vram.get(address)
            }
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => return self.cartridge.ram_read(address),
            RAM0_START..=RAM0_END => self.wram.get(address - RAM0_START),
            RAM_BANK_START..=RAM_BANK_END => {
                let address = self.wram_bank * RAM_BANK_SIZE + (address - RAM_BANK_START);
                self.wram.get(address)
            }
            ECHO_START..=ECHO_END => {
                let address = self.wram_bank * RAM_BANK_SIZE + (address - ECHO_START);
                self.wram.get(address)
            }
            OAM_START..=OAM_END => self.oam.get(address - OAM_START),
            UNUSED_START..=UNUSED_END => Some(&0),
            INTERRUPT_FLAGS => Some(&self.interrupt_flags),
            IO_START..=IO_END => self.io.get(address - IO_START),
            HRAM_START..=HRAM_END => self.hram.get(address - HRAM_START),
            INTERRUPT_ENABLED => Some(&self.interrupt_enable),
            _ => unreachable!(),
        };

        *slot.unwrap_or(&0xFF)
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address);
        let high = self.read_byte(address + 1);

        u16::from_be_bytes([high, low])
    }

    pub fn write_byte<A>(&mut self, address: A, value: u8)
    where
        A: Into<usize>,
    {
        let address = address.into();

        let slot = match address {
            ROM0_START..=ROM0_END | ROM_BANK_START..=ROM_BANK_END => {
                self.cartridge.rom_write(address, value);
                return;
            }
            VRAM_START..=VRAM_END => {
                let address = self.vram_bank * VRAM_SIZE + (address - VRAM_START);
                self.vram.get_mut(address)
            }
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => {
                self.cartridge.ram_write(address, value);
                return;
            }
            RAM0_START..=RAM0_END => self.wram.get_mut(address - RAM0_START),
            RAM_BANK_START..=RAM_BANK_END => {
                let address = self.wram_bank * RAM_BANK_SIZE + (address - RAM_BANK_START);
                self.wram.get_mut(address)
            }
            ECHO_START..=ECHO_END => {
                let address = self.wram_bank * RAM_BANK_SIZE + (address - ECHO_START);
                self.wram.get_mut(address)
            }
            OAM_START..=OAM_END => self.oam.get_mut(address - OAM_START),
            UNUSED_START..=UNUSED_END => return,
            INTERRUPT_FLAGS => Some(&mut self.interrupt_flags),
            IO_START..=IO_END => self.io.get_mut(address - IO_START),
            HRAM_START..=HRAM_END => self.hram.get_mut(address - HRAM_START),
            INTERRUPT_ENABLED => Some(&mut self.interrupt_enable),
            _ => unreachable!(),
        };

        if let Some(slot) = slot {
            *slot = value;
        }
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let [high, low] = value.to_be_bytes();

        self.write_byte(address, low);
        self.write_byte(address + 1, high);
    }

    pub fn stack_push(&mut self, stack_pointer: u16, value: u16) -> u16 {
        let [high, low] = value.to_be_bytes();

        self.write_byte(stack_pointer, high);
        let stack_pointer = stack_pointer.wrapping_sub(1);

        self.write_byte(stack_pointer, low);
        stack_pointer.wrapping_sub(1)
    }

    pub fn stack_pop(&mut self, stack_pointer: u16) -> (u16, u16) {
        let low = self.read_byte(stack_pointer);
        let stack_pointer = stack_pointer.wrapping_add(1);

        let high = self.read_byte(stack_pointer);

        (
            u16::from_be_bytes([high, low]),
            stack_pointer.wrapping_add(1),
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("cartridge error: {0}")]
    CartridgeError(#[from] CartridgeError),
}
