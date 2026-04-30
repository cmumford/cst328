use crate::registers::{
    ChipInfo, FirmwareChecksum, FirmwareVersion, Info1, Info3, Register, Resolutions,
};
use bilge::prelude::*;
use cfg_if::cfg_if;
use core::fmt;
use embedded_hal::{delay::DelayNs, digital::OutputPin};

cfg_if! {
    if #[cfg(feature = "use_sync")] {
        use embedded_hal::i2c::{Error as HalError, SevenBitAddress};
    } else if #[cfg(feature = "use_async")] {
        use embedded_hal_async::i2c::{Error as HalError, SevenBitAddress};
    }
}

pub enum Mode {
    DebugInfo,
    Reset,
    RedoCalibration,
    DeepSleep,
    DebugPoints,
    Normal,
    DebugRawData,
    DebugWrite,
    DebugCalibration,
    DebugDiff,
    Factory,
}

impl From<Mode> for Register {
    fn from(reg: Mode) -> Self {
        match reg {
            Mode::DebugInfo => Self::ModeDebugInfo,
            Mode::Reset => Self::ModeReset,
            Mode::RedoCalibration => Self::ModeRedoCalibration,
            Mode::DeepSleep => Self::ModeDeepSleep,
            Mode::DebugPoints => Self::ModeDebugPoints,
            Mode::Normal => Self::ModeNormal,
            Mode::DebugRawData => Self::ModeDebugRawData,
            Mode::DebugWrite => Self::ModeDebugWrite,
            Mode::DebugCalibration => Self::ModeDebugCalibration,
            Mode::DebugDiff => Self::ModeDebugDiff,
            Mode::Factory => Self::ModeFactory,
        }
    }
}

pub struct DebugInfo {
    pub key_num: u8,
    pub rx_num: u8,
    pub tx_num: u8,
    pub resolutions: Resolutions,
    pub boot_timer: u16,
    pub chip_type: u16,
    pub project_id: u16,
    pub chip_info: ChipInfo,
    pub firmware_version: FirmwareVersion,
    pub firmware_checksum: u32,
}

impl fmt::Debug for DebugInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DebugInfo")
            .field("key_num", &self.key_num)
            .field("rx_num", &self.rx_num)
            .field("tx_num", &self.tx_num)
            .field("resolutions", &self.resolutions)
            .field("boot_timer", &format_args!("0x{:04X}", self.boot_timer))
            .field("chip_type", &format_args!("0x{:04X}", self.chip_type))
            .field("project_id", &self.project_id)
            .field("firmware_version", &self.firmware_version)
            .field("chip_info", &self.chip_info)
            .field(
                "firmware_checksum",
                &format_args!("0x{:08X}", self.firmware_checksum),
            )
            .finish()
    }
}

const I2C_ADDR: SevenBitAddress = 0x1a;
const CST328_RESET_DURATION_LOW_MS: u32 = 10; // TRST: Actually 0.1 ms per datasheet.
const CST328_RESET_DURATION_HIGH_MS: u32 = 300; // TRON: Initialization time after reset.

#[derive(Debug)]
pub enum Error<E: HalError> {
    I2c(E),
    InvalidData,
}

// One single I2cBound definition (no duplicated traits)
cfg_if! {
    if #[cfg(feature = "use_sync")] {
        pub trait I2cBound: embedded_hal::i2c::I2c {}
        impl<T: embedded_hal::i2c::I2c> I2cBound for T {}
    } else if #[cfg(feature = "use_async")] {
        pub trait I2cBound: embedded_hal_async::i2c::I2c {}
        impl<T: embedded_hal_async::i2c::I2c> I2cBound for T {}
    }
}

#[maybe_async_cfg::maybe(
    idents(Cst328),
    sync(feature = "use_sync"),
    async(feature = "use_async")
)]
pub struct Cst328<I2C> {
    i2c: I2C,
}

#[maybe_async_cfg::maybe(
    idents(Cst328),
    sync(feature = "use_sync"),
    async(feature = "use_async")
)]
impl<I2C: I2cBound> Cst328<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    async fn write_reg(&mut self, reg_addr: Register) -> Result<(), Error<I2C::Error>> {
        let reg = (reg_addr as u16).to_be_bytes();
        self.i2c.write(I2C_ADDR, &reg).await.map_err(Error::I2c)
    }

    pub async fn set_mode(&mut self, mode: Mode) -> Result<(), Error<I2C::Error>> {
        self.write_reg(mode.into()).await
    }

    async fn read_reg(&mut self, reg_addr: Register) -> Result<u32, Error<I2C::Error>> {
        let addr = (reg_addr as u16).to_be_bytes();
        let mut response = [0u8; 4];

        self.i2c
            .write_read(I2C_ADDR, &addr, &mut response)
            .await
            .map_err(Error::I2c)?;

        Ok(u32::from_be_bytes(response))
    }

    pub async fn read_debug_info(&mut self) -> Result<DebugInfo, Error<I2C::Error>> {
        let info1 = self.read_info1().await?;
        let resolutions = self.read_info2().await?;
        let info3 = self.read_info3().await?;
        let chip_info = self.read_info4().await?;
        let firmware_version = self.read_firmware_version().await?;
        let firmware_checksum = self.read_firmware_checksum().await?;

        if info3.firmware_checksum() != 0xCACA {
            return Err(Error::InvalidData);
        }
        Ok(DebugInfo {
            key_num: info1.key_num(),
            rx_num: info1.rx_num(),
            tx_num: info1.tx_num(),
            resolutions,
            boot_timer: info3.boot_timer(),
            chip_type: chip_info.ic_type(),
            project_id: chip_info.project_id(),
            chip_info,
            firmware_version,
            firmware_checksum: (firmware_checksum.high() as u32) << 16
                | (firmware_checksum.low() as u32),
        })
    }

    async fn read_info1(&mut self) -> Result<Info1, Error<I2C::Error>> {
        let raw = self.read_reg(Register::Info1).await?;
        Ok(Info1::from(u32::new(raw)))
    }

    async fn read_info2(&mut self) -> Result<Resolutions, Error<I2C::Error>> {
        let raw = self.read_reg(Register::Resolutions).await?;
        Ok(Resolutions::from(u32::new(raw)))
    }

    async fn read_info3(&mut self) -> Result<Info3, Error<I2C::Error>> {
        let raw = self.read_reg(Register::Info3).await?;
        Ok(Info3::from(u32::new(raw)))
    }

    async fn read_info4(&mut self) -> Result<ChipInfo, Error<I2C::Error>> {
        let raw = self.read_reg(Register::ChipInfo).await?;
        Ok(ChipInfo::from(u32::new(raw)))
    }

    async fn read_firmware_version(&mut self) -> Result<FirmwareVersion, Error<I2C::Error>> {
        let raw = self.read_reg(Register::FirmwareVersion).await?;
        Ok(FirmwareVersion::from(u32::new(raw)))
    }

    async fn read_firmware_checksum(&mut self) -> Result<FirmwareChecksum, Error<I2C::Error>> {
        let raw = self.read_reg(Register::FirmwareChecksum).await?;
        Ok(FirmwareChecksum::from(u32::new(raw)))
    }

    /// # Errors
    ///
    /// Will return `Err` upon I2C error.
    pub async fn ping(&mut self) -> Result<(), Error<I2C::Error>> {
        self.i2c.write(I2C_ADDR, &[]).await.map_err(Error::I2c)
    }
}

pub fn reset<O, D>(rst: &mut O, delay: &mut D) -> Result<(), O::Error>
where
    O: OutputPin,
    D: DelayNs,
{
    rst.set_low()?;
    delay.delay_ms(CST328_RESET_DURATION_LOW_MS);
    rst.set_high()?;
    delay.delay_ms(CST328_RESET_DURATION_HIGH_MS);
    Ok(())
}
