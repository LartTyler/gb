use crate::DeviceMode;
use cartridge::{Cartridge, CartridgeError};
use map::*;

pub mod cartridge;
pub mod map;

pub struct Memory {
    pub cartridge: Cartridge,
    pub wram: Bank,
    pub oam: Vec<u8>,
    pub io: Vec<u8>,
    pub hram: Vec<u8>,
    pub interrupt_flags: u8,
    pub interrupts_enabled: u8,
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> Result<Self, MemoryError> {
        let cartridge = Cartridge::new(rom)?;

        let wram_banks = match cartridge.device_mode.into() {
            DeviceMode::Color => 8,
            DeviceMode::Classic => 2,
        };

        Ok(Self {
            cartridge,
            wram: Bank::new(wram_banks, RAM_BANK_SIZE),
            oam: vec![0; OAM_SIZE],
            io: vec![0; IO_SIZE],
            hram: vec![0; HRAM_SIZE],
            interrupt_flags: 0,
            interrupts_enabled: 0,
        })
    }
}

pub struct Bank {
    data: Vec<u8>,
    bank_size: usize,
    current_bank: usize,
}

impl Bank {
    pub fn new(bank_count: usize, bank_size: usize) -> Self {
        Self {
            data: vec![0; bank_count * bank_size],
            bank_size,
            current_bank: 0,
        }
    }

    pub fn get(&self, address: usize) -> Option<&u8> {
        self.data.get(self.map_address(address, self.current_bank))
    }

    pub fn get_from(&self, bank: usize, address: usize) -> Option<&u8> {
        self.data.get(self.map_address(address, bank))
    }

    pub fn get_mut(&mut self, address: usize) -> Option<&mut u8> {
        let address = self.map_address(address, self.current_bank);
        self.data.get_mut(address)
    }

    pub fn get_from_mut(&mut self, bank: usize, address: usize) -> Option<&mut u8> {
        let address = self.map_address(address, bank);
        self.data.get_mut(address)
    }

    pub fn set(&mut self, address: usize, value: u8) {
        let slot = self.get_mut(address);

        if let Some(slot) = slot {
            *slot = value;
        }
    }

    fn map_address(&self, address: usize, bank: usize) -> usize {
        address + bank * self.bank_size
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("cartridge error: {0}")]
    CartridgeError(#[from] CartridgeError),
}
