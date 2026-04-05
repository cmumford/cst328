use embedded_hal::i2c::I2c;

#[derive(Debug)]
pub enum Error<E: embedded_hal::i2c::Error> {
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

pub struct CST328<I2C> {
    i2c: I2C,
}

impl<I2C: I2c> CST328<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn read_touch(&mut self) -> Result<TouchData, Error<I2C::Error>> {
        let mut buf = [0u8; 32];

        self.i2c
            .write_read(0x15, &[0xD0, 0x00], &mut buf)
            .map_err(Error::I2c)?;

        parse_touch_data(&buf).map_err(|_| Error::InvalidData)
    }
}

fn parse_touch_data(buf: &[u8]) -> Result<TouchData, ()> {
    let num_points = (buf[0x05] & 0x0F).min(5);

    let mut data = TouchData {
        num_points,
        points: [TouchPoint::default(); 5],
    };

    for i in 0..num_points as usize {
        let base = i * 6;

        let x = ((buf[base + 0x01] as u16) << 4) | (buf[base + 0x03] as u16 & 0x0F);
        let y = ((buf[base + 0x02] as u16) << 4) | (buf[base + 0x03] as u16 >> 4);

        data.points[i] = TouchPoint {
            id: buf[base] >> 4,
            x,
            y,
            weight: buf[base + 0x04],
        };
    }

    Ok(data)
}
