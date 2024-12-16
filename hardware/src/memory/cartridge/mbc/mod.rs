use super::OFFSET_CONTROLLER_TYPE;
use crate::memory::map::{EXTERNAL_RAM_SIZE, EXTERNAL_RAM_START, ROM_BANK_SIZE};
use derive_more::derive::Display;
use mbc0::Mbc0;
use mbc1::Mbc1;
use mbc5::Mbc5;

pub mod mbc0;
pub mod mbc1;
pub mod mbc5;

/// Shared trait for all memory bank controller (MBC) implementations.
///
/// Please refer to each MBC's implementation for more information on special memory sectors and
/// read / write behaviors. MBCs are... complicated.
pub trait ControllerAccess {
    /// Reads a value from ROM.
    ///
    /// Reads outside the supported range should return `0xFF`.
    fn rom_read(&self, address: usize) -> u8;

    /// "Writes" a value to the MBC registers mapped to ROM memory segments (if supported by the
    /// MBC).
    ///
    /// Each implementation will handle this drastically differently. Please refer to each MBC's
    /// documentation for a more in-depth explanation on what writing to read-only memory means
    /// (and why it's actually more confusing than you'd assume).
    fn rom_write(&mut self, address: usize, value: u8);

    /// Reads a value from the external RAM stored on the cartridge.
    ///
    /// In (almost?) all cases, RAM must be abled before it can be read. Reads to disabled RAM, or
    /// RAM addresses that are out of range, should return `0xFF`.
    fn ram_read(&self, address: usize) -> u8;

    /// Writes a value to the external RAM stored on the cartridge.
    ///
    /// In (almost?) all cases, RAM must be enabled before it can be written. Writes to disabled
    /// RAM may be ignored, or may be interpreted in MBC-specific ways. Refer to each MBC's
    /// documentation for information on how this is handled.
    fn ram_write(&mut self, address: usize, value: u8);

    /// Returns the `Controller` variant that this implementation supports. Mostly used for
    /// debugging.
    fn get_controller_type(&self) -> Controller;
}

#[derive(Debug, Copy, Clone, Display)]
pub enum Controller {
    #[display("MBC0")]
    Mbc0,
    #[display("MBC1")]
    Mbc1,
    #[display("MBC3")]
    Mbc3,
    #[display("MBC5")]
    Mbc5,
}

impl Controller {
    pub fn create(&self, rom: Vec<u8>) -> Box<dyn ControllerAccess> {
        match self {
            Self::Mbc0 => Box::new(Mbc0::new(rom)),
            Self::Mbc1 => Box::new(Mbc1::new(rom)),
            Self::Mbc5 => Box::new(Mbc5::new(rom)),
            _ => todo!(),
        }
    }

    pub fn create_for_rom(rom: Vec<u8>) -> Result<Box<dyn ControllerAccess>, CreateError> {
        let variant = match rom[OFFSET_CONTROLLER_TYPE] {
            0x00 => Self::Mbc0,
            0x01..=0x03 => Self::Mbc1,
            0x0F..=0x13 => Self::Mbc3,
            0x19..=0x1E => Self::Mbc5,
            x => return Err(CreateError::UnsupportedControllerType(x)),
        };

        Ok(variant.create(rom))
    }
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub enum CreateError {
    #[error("unsupported controller type id {0}")]
    UnsupportedControllerType(u8),
}

/// Maps a normal ROM address to an absolute banked address.
///
/// This function assumes banks are stored in a contiguous range, e.g. a `Vec`, _including_ ROM0.
fn map_rom_address(bank: usize, address: usize) -> usize {
    bank * ROM_BANK_SIZE + (address - ROM_BANK_SIZE)
}

/// Maps a normal RAM address to an absolute banked address.
///
/// This function assumes all banks are stored in a contiguous range, e.g. a `Vec`.
fn map_ram_address(bank: usize, address: usize) -> usize {
    bank * EXTERNAL_RAM_SIZE + (address - EXTERNAL_RAM_START)
}
