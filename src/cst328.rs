use crate::registers::{
    ChipInfo, FirmwareChecksum, FirmwareVersion, Info1, Info3, Register, Resolutions,
};
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
    pub info3: Info3,
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
            .field("info3", &self.info3)
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

macro_rules! map_struct {
    ($buffer:expr, $offset:expr, $type:ty) => {{
        let chunk: [u8; 4] = $buffer[$offset..$offset + 4]
            .try_into()
            .expect("Buffer overflow mapping struct");
        <$type>::from(u32::from_le_bytes(chunk))
    }};
}

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

    pub async fn read_debug_info_new(&mut self) -> Result<DebugInfo, Error<I2C::Error>> {
        let mut response = [0u8; 3 * core::mem::size_of::<u32>()];

        // The first three registers are contiguous and can be read in a single transaction.
        let addr = (Register::Info1 as u16).to_be_bytes();
        self.i2c
            .write_read(I2C_ADDR, &addr, &mut response)
            .await
            .map_err(Error::I2c)?;
        let info1 = map_struct!(response, 0, Info1);
        let resolutions: Resolutions = map_struct!(response, 4, Resolutions);
        let info3 = map_struct!(response, 8, Info3);

        let addr = (Register::ChipInfo as u16).to_be_bytes();
        self.i2c
            .write_read(I2C_ADDR, &addr, &mut response)
            .await
            .map_err(Error::I2c)?;
        let chip_info = map_struct!(response, 0, ChipInfo);
        let firmware_version = map_struct!(response, 4, FirmwareVersion);
        let firmware_checksum = map_struct!(response, 8, FirmwareChecksum);

        if info3.firmware_checksum() != 0xCACA {
            return Err(Error::InvalidData);
        }
        Ok(DebugInfo {
            key_num: info1.key_num(),
            rx_num: info1.rx_num(),
            tx_num: info1.tx_num(),
            resolutions,
            info3,
            chip_info,
            firmware_version,
            firmware_checksum: u32::from(firmware_checksum.high()) << 16
                | u32::from(firmware_checksum.low()),
        })
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
