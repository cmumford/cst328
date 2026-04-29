use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "use_sync")] {
        use embedded_hal::i2c::{Error as HalError, SevenBitAddress};
    } else if #[cfg(feature = "use_async")] {
        use embedded_hal_async::i2c::{Error as HalError, SevenBitAddress};
    }
}

const I2C_ADDR: SevenBitAddress = 0x1a;

#[derive(Debug)]
pub enum Error<E: HalError> {
    I2c(E),
    InvalidData,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TouchPoint {
    pub id: u8,
    pub x: u16,
    pub y: u16,
    pub weight: u8,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TouchData {
    pub num_points: u8,
    pub points: [TouchPoint; 5],
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

    /// # Errors
    ///
    /// Will return `Err` upon I2C error.
    pub async fn read_touch(&mut self) -> Result<TouchData, Error<I2C::Error>> {
        let mut buf = [0u8; 32];

        self.i2c
            .write_read(I2C_ADDR, &[0xD0, 0x00], &mut buf)
            .await
            .map_err(Error::I2c)?;

        Ok(parse_touch_data(&buf))
    }

    /// # Errors
    ///
    /// Will return `Err` upon I2C error.
    pub async fn ping(&mut self) -> Result<(), Error<I2C::Error>> {
        self.i2c.write(I2C_ADDR, &[]).await.map_err(Error::I2c)
    }
}

fn parse_touch_data(buf: &[u8]) -> TouchData {
    let num_points = (buf[0x05] & 0x0F).min(5);

    let mut data = TouchData {
        num_points,
        points: [TouchPoint::default(); 5],
    };

    for i in 0..num_points as usize {
        let base = i * 6;
        let x = (u16::from(buf[base + 0x01]) << 4) | (u16::from(buf[base + 0x03]) & 0x0F);
        let y = (u16::from(buf[base + 0x02]) << 4) | (u16::from(buf[base + 0x03]) >> 4);

        data.points[i] = TouchPoint {
            id: buf[base] >> 4,
            x,
            y,
            weight: buf[base + 0x04],
        };
    }

    data
}
