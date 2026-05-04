use bilge::prelude::*;
use core::fmt;

/// The register addresses for the CST328.
#[allow(dead_code)]
#[repr(u16)]
pub enum Register {
    /// The first finger's register.
    Finger1 = 0xD000,
    /// Debug register for: Key, TX and RX numbers
    Info1 = 0xD1F4,
    /// Debug register for: X/Y resolutions
    Resolutions = 0xD1F8,
    /// Debug register for: Firmware checksum, boot loader window time
    Info3 = 0xD1FC,
    /// Debug register for: Chip type and Project ID
    ChipInfo = 0xD204,
    /// Debug register for: Firmware version
    FirmwareVersion = 0xD208,
    /// Debug register for: Firmware checksum
    FirmwareChecksum = 0xD20C,
    /// Debug register for: enter debug info mode.
    ModeDebugInfo = 0xD101,
    /// Debug register for: enter reset mode.
    ModeReset = 0xD102,
    /// Debug register for: enter redo calibration mode.
    ModeRedoCalibration = 0xD104,
    /// Debug register for: enter deep sleep mode.
    ModeDeepSleep = 0xD105,
    /// Debug register for: enter debug points mode.
    ModeDebugPoints = 0xD108,
    /// Debug register for: enter normal mode.
    ModeNormal = 0xD109,
    /// Debug register for: enter debug raw data mode.
    ModeDebugRawData = 0xD10A,
    /// Debug register for: enter debug write mode.
    ModeDebugWrite = 0xD10B,
    /// Debug register for: enter debug calibration mode.
    ModeDebugCalibration = 0xD10C,
    /// Debug register for: enter debug diff mode.
    ModeDebugDiff = 0xD10D,
    /// Debug register for: enter factory mode.
    ModeFactory = 0xD119,
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct Info1 {
    pub tx_num: u8,
    pub nc: u8,
    pub rx_num: u8,
    pub key_num: u8,
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct Resolutions {
    pub y: u16,
    pub x: u16,
}

#[bitsize(32)]
#[derive(FromBits, PartialEq)]
pub struct Info3 {
    pub boot_timer: u16,        // Boot loader window time.
    pub firmware_checksum: u16, // always 0xCACA.
}

impl fmt::Debug for Info3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Info3")
            .field(
                "firmware_checksum",
                &format_args!("0x{:04X}", self.firmware_checksum()),
            )
            .field("boot_timer", &format_args!("0x{:04X}", self.boot_timer()))
            .finish()
    }
}

#[bitsize(32)]
#[derive(FromBits, PartialEq)]
pub struct ChipInfo {
    pub project_id: u16,
    pub ic_type: u16,
}

impl fmt::Debug for ChipInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChipInfo")
            .field("ic_type", &format_args!("0x{:04X}", self.ic_type()))
            .field("project_id", &format_args!("0x{:04X}", self.project_id()))
            .finish()
    }
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct FirmwareVersion {
    pub build: u16,
    pub minor: u8,
    pub major: u8,
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct FirmwareChecksum {
    pub low: u16,
    pub high: u16,
}

#[bitsize(40)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct FingerEntry {
    pub pressure: u8,
    pub y_pos_low: u4,  // Low 4 bits of X position.
    pub x_pos_low: u4,  // Low 4 bits of Y position.
    pub y_pos_high: u8, // High 8 bits (out of 12) of Y position.
    pub x_pos_high: u8, // High 8 bits (out of 12) of X position.
    pub status: u4,     // Touch(0x06) or lift.
    pub id: u4,         // Finger ID.
}
