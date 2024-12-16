use super::{map_ram_address, map_rom_address, Controller, ControllerAccess};
use crate::memory::cartridge::read_ram_size;

pub struct Mbc5 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_enabled: bool,
}

impl Mbc5 {
    pub fn new(rom: Vec<u8>) -> Self {
        let ram_size = read_ram_size(&rom).expect("Unsupported RAM size");
        let ram = vec![0; ram_size];

        // TODO MBC5s also include rumble pack support. I'm not sure that's really relevant for
        // emulation (maybe setting up some sort of screen shake to mimic the rumble effect?), but
        // I might need to tweak things to account for writes to the registers it uses so it
        // doesn't affect the current RAM bank.

        Self {
            rom,
            ram,
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
        }
    }
}

impl ControllerAccess for Mbc5 {
    fn rom_read(&self, address: usize) -> u8 {
        let address = map_rom_address(self.rom_bank, address);
        *self.rom.get(address).unwrap_or(&0xFF)
    }

    fn rom_write(&mut self, address: usize, value: u8) {
        match address {
            0x0000..0x2000 => self.ram_enabled = value & 0xA != 0,
            0x2000..0x3000 => self.rom_bank = (self.rom_bank & 0x100) | (value as usize),
            0x3000..0x4000 => self.rom_bank = (self.rom_bank & 0xFF) | (value & 1) as usize,
            0x4000..0x6000 => self.ram_bank = value as usize & 0x0F,
            _ => (),
        };
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
        Controller::Mbc5
    }
}
