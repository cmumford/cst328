use bilge::prelude::*;

#[repr(u16)]
pub enum Register {
    Info1 = 0xD1F4,
    Resolutions = 0xD1F8,
    Info3 = 0xD1FC,
    ChipInfo = 0xD204,
    FirmwareVersion = 0xD208,
    FirmwareChecksum = 0xD20C,
    ModeDebugInfo = 0xD101,
    ModeReset = 0xD102,
    ModeRedoCalibration = 0xD104,
    ModeDeepSleep = 0xD105,
    ModeDebugPoints = 0xD108,
    ModeNormal = 0xD109,
    ModeDebugRawData = 0xD10A,
    ModeDebugWrite = 0xD10B,
    ModeDebugCalibration = 0xD10C,
    ModeDebugDiff = 0xD10D,
    ModeFactory = 0xD119,
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct Info1 {
    pub key_num: u8,
    pub rx_num: u8,
    pub reserved: u8,
    pub tx_num: u8,
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct Resolutions {
    pub x: u16,
    pub y: u16,
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct Info3 {
    pub firmware_checksum: u16, // always 0xCACA.
    pub boot_timer: u16,        // Boot loader window time.
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct ChipInfo {
    pub ic_type: u16,
    pub project_id: u16,
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct FirmwareVersion {
    pub major: u8,
    pub minor: u8,
    pub build: u16,
}

#[bitsize(32)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct FirmwareChecksum {
    pub high: u16,
    pub low: u16,
}

#[bitsize(40)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct Finger {
    pub id: u4,         // Finger ID.
    pub status: u4,     // Touch(0x06) or lift.
    pub x_pos_high: u8, // High 8 bits of X position.
    pub y_pos_high: u8, // High 8 bits of Y position.
    pub x_pos_low: u4,  // Low 4 bits of X position.
    pub y_pos_low: u4,  // Low 4 bits of Y position.
    pub pressure: u8,
}
