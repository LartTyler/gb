use std::fmt::Debug;

use crate::memory::map::EXTERNAL_RAM_SIZE;
use map::*;
use mbc::{Controller, ControllerAccess};

pub mod map;
pub mod mbc;

#[derive(Debug, Copy, Clone)]
pub enum SupportedDeviceMode {
    Color,
    Classic,
    Any,
}

pub struct Cartridge {
    pub title: String,
    pub device_mode: SupportedDeviceMode,
    pub licensee_id: u16,
    pub sgb_support: bool,
    pub version: u8,
    pub controller: Box<dyn ControllerAccess>,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Result<Self, CartridgeError> {
        Ok(Self {
            title: read_title(&rom),
            device_mode: read_supported_mode(&rom),
            licensee_id: read_licensee_id(&rom),
            sgb_support: read_sgb_support_flag(&rom),
            version: read_version(&rom),
            controller: Controller::create_for_rom(rom)?,
        })
    }
}

impl Debug for Cartridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cartridge")
            .field("title", &self.title)
            .field("device_mode", &self.device_mode)
            .field("licensee_id", &self.licensee_id)
            .field("sgb_support", &self.sgb_support)
            .field("version", &self.version)
            .field("controller", &self.controller.get_controller_type())
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CartridgeError {
    #[error("controller error: {0}")]
    ControllerError(#[from] mbc::CreateError),
}

macro_rules! check_offset {
    ($rom:ident [ $offset:expr ]) => {
        assert!(
            $rom.len() > $offset,
            "Invalid ROM: cannot read to offset {:#04X}",
            $offset
        );
    };
}

/// Retrieves the ROM's title.
///
/// A title begins at [`OFFSET_TITLE_START`], and may be up to either 11 of 16 ASCII characters
/// long, depending on the return value of [`read_title_max_length()`].
///
/// Titles are padded with null bytes if the length of the title is less than the maximum length.
/// The return value of this function _does not_ include the null padding, and ignores bytes
/// starting with the first null byte encountered.
pub fn read_title(rom: &[u8]) -> String {
    let title_len = read_title_max_length(rom);
    check_offset!(rom[OFFSET_TITLE_START + title_len]);

    let mut title = String::with_capacity(title_len);

    for i in 0..title_len {
        match rom[OFFSET_TITLE_START + i] {
            0 => break,
            c => title.push(c as char),
        };
    }

    title
}

/// Returns the maximum possible title length, based on the value of [`OFFSET_GBC_SUPPORT_TYPE`].
///
/// Pre-GBC ROMs could include titles up to 16 characters long. For the Gameboy Color, the title
/// length was reduced to 11 characters to make room for two new header fields: manufacturer code
/// and [support type](OFFSET_GBC_SUPPORT_TYPE).
pub fn read_title_max_length(rom: &[u8]) -> usize {
    check_offset!(rom[OFFSET_GBC_SUPPORT_TYPE]);

    if rom[OFFSET_GBC_SUPPORT_TYPE] & 0x80 != 0 {
        11
    } else {
        16
    }
}

/// Retrieves which hardware the ROM supports.
///
/// The support mode is determined by examining the byte at [`OFFSET_GBC_SUPPORT_TYPE`]:
///     - A value of `0x80` means that the ROM supports Color hardware enhancements, but is
///     backwards compatible.
///     - A value of `0xC0` means that the ROM _only_ supports Gameboy Color hardware.
///     - Any other value means that the ROM was _intended_ to run on the Classic hardware.
///
/// Emphases is placed on "intended" because the Color is both capable of and happy to run Classic
/// games without issue. There are some differences in how the contents of the ROM are interpreted
/// (e.g. title length, color palettes, etc.), but that appears to be the only difference.
pub fn read_supported_mode(rom: &[u8]) -> SupportedDeviceMode {
    check_offset!(rom[OFFSET_GBC_SUPPORT_TYPE]);

    match rom[OFFSET_GBC_SUPPORT_TYPE] {
        0x80 => SupportedDeviceMode::Any,
        0xC0 => SupportedDeviceMode::Color,
        _ => SupportedDeviceMode::Classic,
    }
}

/// Retrieves the licensee ID.
///
/// The Licensee ID is used to determine which company created the ROM. Translating licensee IDs to
/// company names is (at the moment) beyond the scope of this project, but a table of IDs and
/// company names can be found
/// [here](https://gbdev.io/pandocs/The_Cartridge_Header.html#0144-0145---new-licensee-code) and
/// [here](https://raw.githubusercontent.com/gb-archive/salvage/master/txt-files/gbrom.txt).
pub fn read_licensee_id(rom: &[u8]) -> u16 {
    check_offset!(rom[OFFSET_OLD_LICENSEE]);

    match rom[OFFSET_OLD_LICENSEE] {
        0x33 => {
            check_offset!(rom[OFFSET_NEW_LICENSEE_LOW]);
            u16::from_be_bytes([rom[OFFSET_NEW_LICENSEE_HIGH], rom[OFFSET_NEW_LICENSEE_LOW]])
        }
        x => x as u16,
    }
}

/// Retrieves the status of Super Gameboy Support.
///
/// If set to any value other than `0x03`, the SGB will ignore any command packets.
pub fn read_sgb_support_flag(rom: &[u8]) -> bool {
    check_offset!(rom[OFFSET_SGB_SUPPORT_FLAG]);
    rom[OFFSET_SGB_SUPPORT_FLAG] == 0x03
}

/// Retrieves the ROM's version.
///
/// For most ROMs, this seems to be set to `0x00`.
pub fn read_version(rom: &[u8]) -> u8 {
    check_offset!(rom[OFFSET_VERSION]);
    rom[OFFSET_VERSION]
}

/// Retrieves the size of the cartridge RAM.
///
/// A map of RAM sizes can be found
/// [here](https://gbdev.io/pandocs/The_Cartridge_Header.html#0149--ram-size).
pub fn read_ram_size(rom: &[u8]) -> Result<usize, u8> {
    check_offset!(rom[OFFSET_RAM_SIZE]);

    let val = match rom[OFFSET_RAM_SIZE] {
        0 => 0,

        // 2KB, however Pandocs lists this as never used
        1 => 2048,

        // 8KB (1 bank)
        2 => EXTERNAL_RAM_SIZE,

        // 32KB (4 banks)
        3 => 4 * EXTERNAL_RAM_SIZE,

        // 128KB (16 banks)
        4 => 16 * EXTERNAL_RAM_SIZE,

        // 64KB (8 banks); TCAGBD: Used by "Pokemon Crystal (J)"
        5 => 8 * EXTERNAL_RAM_SIZE,

        x => return Err(x),
    };

    Ok(val)
}
