use super::{Controller, ControllerAccess};

pub struct Mbc0 {
    rom: Vec<u8>,
}

impl Mbc0 {
    pub fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }
}

impl ControllerAccess for Mbc0 {
    fn rom_read(&self, address: usize) -> u8 {
        *self.rom.get(address).unwrap_or(&0xFF)
    }

    fn rom_write(&mut self, _address: usize, _value: u8) {}

    fn ram_read(&self, _address: usize) -> u8 {
        0xFF
    }

    fn ram_write(&mut self, _address: usize, _value: u8) {}

    fn get_controller_type(&self) -> Controller {
        Controller::Mbc0
    }
}
