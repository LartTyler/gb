use crate::{memory::Bank, DeviceMode, VRAM_SIZE};

pub const REGISTER_LCD_Y_COORD: usize = 0xFF44;
pub const REGISTER_LCD_Y_COMPARE: usize = 0xFF45;
pub const REGISTER_LCD_STATUS: usize = 0xFF41;

pub struct Video {
    pub vram: Bank,
    pub current_line: u8,
    pub current_line_compare: u8,
    pub status_register: u8,
    pub has_vblank_interrupt: bool,
    pub has_stat_interrupt: bool,
    mode: Mode,
    total_dots: u16,
    remaining_dots: u16,
    speed_multiplier: u16,
}

impl Video {
    pub fn new(device_mode: DeviceMode) -> Self {
        let mode = Mode::OamScan;
        let total_dots = mode.get_duration(0);

        let vram_banks = match device_mode {
            DeviceMode::Color => 2,
            DeviceMode::Classic => 1,
        };

        let mut inst = Self {
            vram: Bank::new(vram_banks, VRAM_SIZE),
            current_line: 0,
            current_line_compare: 0,
            status_register: mode as u8,
            speed_multiplier: 1,
            remaining_dots: total_dots,
            has_stat_interrupt: false,
            has_vblank_interrupt: false,
            total_dots,
            mode,
        };

        // The LYC status flag always starts `true` since the current line and compare registers
        // start at zero.
        inst.set_flag(Flag::LycStatus, true);

        inst
    }

    pub fn process(&mut self, delta: u8) {
        // A VBlank interrupt is only requested on the tick that we enter vblank, and should be
        // cleared on the next one.
        self.has_vblank_interrupt = false;

        let delta = delta as u16 * self.speed_multiplier;

        // Because we only tick the video device after each instruction, we might end up with a
        // delta that is greater than the remaining dots for the current mode. In that case,
        // `overflow` will be non-zero, telling us we need to subtract it from the next mode's
        // remaining_dots when we switch to it.
        let overflow = delta.saturating_sub(self.remaining_dots);
        let new_dots = self.remaining_dots.saturating_sub(delta);

        if new_dots == 0 {
            // Advance the line counter by 1 every time we complete a vertical or horizontal blank.
            if matches!(self.mode, Mode::HorizontalBlank | Mode::VerticalBlank) {
                self.current_line += 1;
            }

            self.mode = self.mode.next(self.current_line);
            self.total_dots = self.mode.get_duration(self.total_dots);
            self.remaining_dots = self.total_dots - overflow;

            self.has_vblank_interrupt = matches!(self.mode, Mode::VerticalBlank);
            self.has_stat_interrupt = match self.mode {
                Mode::VerticalBlank => self.get_flag(Flag::VblankInterrupt),
                Mode::HorizontalBlank => self.get_flag(Flag::HblankInterrupt),
                Mode::OamScan => self.get_flag(Flag::OamInterrupt),
                _ => false,
            };
        }

        self.set_flag(
            Flag::LycStatus,
            self.current_line == self.current_line_compare,
        );
    }

    pub fn write_status_register(&mut self, value: u8) {
        // The lower 3 bits of the LCD STAT register are not writable, so we need to ignore them.
        self.status_register = value & 0b1111_1000;
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.status_register |= flag as u8;
        } else {
            self.status_register &= !(flag as u8);
        }
    }

    fn get_flag(&self, flag: Flag) -> bool {
        self.status_register & (flag as u8) > 0
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Flag {
    CurrentMode = 0b0000_0011,
    LycStatus = 0b0000_0100,
    HblankInterrupt = 0b0000_1000,
    VblankInterrupt = 0b0001_0000,
    OamInterrupt = 0b0010_0000,
    LycInterrupt = 0b0100_0000,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Mode {
    HorizontalBlank = 0,
    VerticalBlank = 1,
    OamScan = 2,
    Draw = 3,
}

impl Mode {
    const OAM_DOTS: u16 = 80;
    const VBLANK_DOTS: u16 = 4560;
    const HBLANK_MAX_DOTS: u16 = 204;
    const DRAW_MIN_DOTS: u16 = 172;
    const LINE_COUNT: u8 = 154;
    const FIRST_VBLANK_LINE: u8 = 144;

    pub fn next(&self, current_line: u8) -> Self {
        match self {
            Self::OamScan => Self::Draw,
            Self::Draw => Self::HorizontalBlank,
            Self::HorizontalBlank if current_line >= Self::FIRST_VBLANK_LINE => Self::OamScan,
            Self::HorizontalBlank => Self::VerticalBlank,

            // Vertical blanking continues for several lines past the "end" of the display.
            Self::VerticalBlank if current_line >= Self::LINE_COUNT => Self::OamScan,
            Self::VerticalBlank => Self::VerticalBlank,
        }
    }

    pub fn get_duration(&self, previous_dots: u16) -> u16 {
        match self {
            Self::OamScan => Self::OAM_DOTS,
            Self::VerticalBlank => Self::VBLANK_DOTS,
            Self::HorizontalBlank => Self::HBLANK_MAX_DOTS - (previous_dots - Self::DRAW_MIN_DOTS),
            Self::Draw => Self::DRAW_MIN_DOTS,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum ControlFlag {
    BackgroundWindowPriority = 0b0000_0001,
    ObjectsEnabled = 0b0000_0010,
    ObjectSize = 0b0000_0100,
    BackgroundTileMapArea = 0b0000_1000,
    BackgroundWindowDataArea = 0b0001_0000,
    WindowEnabled = 0b0010_0000,
    WindowTileMapArea = 0b0100_0000,
    Enabled = 0b1000_0000,
}
