use core::panic;

use crate::memory::{
    cartridge::read_ram_size,
    map::{EXTERNAL_RAM_SIZE, ROM0_END, ROM0_START, ROM_BANK_END, ROM_BANK_START},
};

use super::{map_ram_address, map_rom_address, Controller, ControllerAccess};

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_enabled: bool,
    advanced_bank_mode: bool,
}

impl Mbc1 {
    pub fn new(rom: Vec<u8>) -> Self {
        let ram_size = read_ram_size(&rom).expect("Unsupported RAM size");
        let ram = vec![0; ram_size];

        Self {
            rom,
            ram,
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            advanced_bank_mode: false,
        }
    }
}

impl ControllerAccess for Mbc1 {
    fn rom_read(&self, address: usize) -> u8 {
        match address {
            ROM0_START..=ROM0_END => *self.rom.get(address).unwrap_or(&0xFF),
            ROM_BANK_START..=ROM_BANK_END => {
                let address = map_rom_address(self.rom_bank, address);
                *self.rom.get(address).unwrap_or(&0xFF)
            }
            _ => panic!("ROM read out of range for MBC1: {address:#X}"),
        }
    }

    fn rom_write(&mut self, address: usize, value: u8) {
        match address {
            0x0000..0x2000 => self.ram_enabled = value & 0x0A != 0,
            0x2000..0x4000 => self.rom_bank = (value & 0b1_1111).max(1) as usize,
            0x4000..0x6000 => {
                let bits = (value & 0b11) as usize;

                if self.advanced_bank_mode {
                    // "Large RAM" carts allow RAM bank swapping in advanced mode
                    if self.ram.len() >= EXTERNAL_RAM_SIZE {
                        self.ram_bank = bits;
                    } else {
                        // "Large ROM" carts allow special case ROM banking in advanced mode that I
                        // really do not understand at the moment...
                        unimplemented!("Support for 'large ROM' carts is not yet complete");
                    }
                } else {
                    self.rom_bank = self.rom_bank & 0b1_1111 | (bits << 5);
                }
            }
            0x6000..0x8000 => self.advanced_bank_mode = value & 0b1 != 0,
            _ => panic!("ROM write out of range for MBC1: {address:#X}"),
        }
    }

    fn ram_read(&self, address: usize) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        let address = map_ram_address(self.ram_bank, address);
        *self.ram.get(address).unwrap_or(&0xFF)
    }

    fn ram_write(&mut self, address: usize, value: u8) {
        if !self.ram_enabled {
            return;
        }

        let address = map_ram_address(self.ram_bank, address);
        let slot = self.ram.get_mut(address);

        if let Some(slot) = slot {
            *slot = value;
        }
    }

    fn get_controller_type(&self) -> Controller {
        Controller::Mbc1
    }
}
