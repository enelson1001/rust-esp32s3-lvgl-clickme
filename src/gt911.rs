/// A minimal implementation of the GT911 to work with Lvgl since Lvgl only uses a single touch point
/// The default orientation and size are based on the aliexpress ESP 7 inch capactive touch development
/// board model ESP-8048S070C
use embedded_hal::{
    delay::DelayUs,
    digital::OutputPin,
    i2c::{I2c, SevenBitAddress},
};

const DEFAULT_GT911_ADDRESS: u8 = 0x5d;

/// Documented registers of the device
#[allow(dead_code)]
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
enum Reg {
    ProductId = 0x8140,
    PointInfo = 0x814E,
    Point1 = 0x814F,
}

/// Represents the orientation of the device
#[derive(Copy, Clone, Debug)]
pub enum Orientation {
    Portrait, // Do Not use
    Landscape,
    InvertedPortrait, // Do Not use
    InvertedLandscape,
}

/// Represents the dimensions of the device
#[derive(Copy, Clone, Debug)]
pub struct Dimension {
    pub height: u16,
    pub width: u16,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TouchPoint {
    pub id: u8,
    pub x: u16,
    pub y: u16,
    pub size: u16,
}

/// Driver representation holding:
///
/// - The I2C Slave address of the GT911
/// - The I2C Bus used to communicate with the GT911
/// - The reset pin on the GT911
/// - The delay used by the reset pin to reset the GT911
/// - The screen/panel orientation
/// - The scree/panel dimesions
#[derive(Clone, Debug)]
pub struct GT911<I2C, RST, DELAY>
where
    I2C: I2c<SevenBitAddress>,
    RST: OutputPin,
    DELAY: DelayUs,
{
    address: u8,
    i2c: I2C,
    reset_pin: RST,
    delay: DELAY,
    orientation: Orientation,
    size: Dimension,
}

impl<I2C, RST, DELAY> GT911<I2C, RST, DELAY>
where
    I2C: I2c<SevenBitAddress>,
    RST: OutputPin,
    DELAY: DelayUs,
{
    //pub fn new(i2c: I2C) -> Self {
    pub fn new(i2c: I2C, reset_pin: RST, delay: DELAY) -> Self {
        Self {
            address: DEFAULT_GT911_ADDRESS,
            i2c,
            reset_pin,
            delay,
            orientation: Orientation::Landscape,
            size: Dimension {
                height: 480,
                width: 800,
            },
        }
    }

    pub fn reset(&mut self) -> Result<(), <RST as embedded_hal::digital::ErrorType>::Error> {
        //println!("======= Resetting GT911 =======");
        self.reset_pin.set_low()?;
        self.delay.delay_us(100);
        self.reset_pin.set_high()?;
        self.delay.delay_ms(100);

        Ok(())
    }

    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    pub fn set_size(&mut self, height: u16, width: u16) {
        self.size = Dimension { height, width };
    }

    // Useful function to determine if you are communicating with GT911, The GT911 must first be reset.
    // The return string should be - 911
    pub fn read_product_id(
        &mut self,
    ) -> Result<String, <I2C as embedded_hal::i2c::ErrorType>::Error> {
        let mut rx_buf: [u8; 4] = [0; 4];

        let product_id_reg: u16 = Reg::ProductId as u16;

        let hi_byte: u8 = (product_id_reg >> 8).try_into().unwrap();
        let lo_byte: u8 = (product_id_reg & 0xFF).try_into().unwrap();
        let tx_buf: [u8; 2] = [hi_byte, lo_byte];

        self.i2c.write_read(self.address, &tx_buf, &mut rx_buf)?;

        Ok(std::str::from_utf8(&rx_buf).unwrap().to_string())
    }

    pub fn read_touch(
        &mut self,
    ) -> Result<Option<TouchPoint>, <I2C as embedded_hal::i2c::ErrorType>::Error> {
        let mut rx_buf: [u8; 1] = [0xFF];

        let point_info_reg: u16 = Reg::PointInfo as u16;
        let hi_byte: u8 = (point_info_reg >> 8).try_into().unwrap();
        let lo_byte: u8 = (point_info_reg & 0xFF).try_into().unwrap();
        let tx_buf: [u8; 2] = [hi_byte, lo_byte];

        self.i2c.write_read(self.address, &tx_buf, &mut rx_buf)?;

        let point_info = rx_buf[0];
        let buffer_status = point_info >> 7 & 1u8;
        let touches = point_info & 0x7;

        //println!("point info = {:x?}", point_info);
        //println!("bufferStatus = {:?}", point_info >> 7 & 1u8);
        //println!("largeDetect = {:?}", point_info >> 6 & 1u8);
        //println!("proximityValid = {:?}", point_info >> 5 & 1u8);
        //println!("HaveKey = {:?}", point_info >> 4 & 1u8);
        //println!("touches = {:?}", point_info & 0xF);

        let is_touched: bool = buffer_status == 1 && touches > 0;

        let mut tp: TouchPoint = TouchPoint {
            id: 0,
            x: 0,
            y: 0,
            size: 0,
        };

        if is_touched {
            tp = self.read_touch_point(Reg::Point1 as u16)?;
        }

        // Reset point_info register after reading it
        let tx_buf: [u8; 3] = [hi_byte, lo_byte, 0u8];
        self.i2c.write(self.address, &tx_buf)?;

        Ok(if is_touched { Some(tp) } else { None })
    }

    pub fn read_touch_point(
        &mut self,
        point_register: u16,
    ) -> Result<TouchPoint, <I2C as embedded_hal::i2c::ErrorType>::Error> {
        let hi_byte: u8 = (point_register >> 8).try_into().unwrap();
        let lo_byte: u8 = (point_register & 0xFF).try_into().unwrap();
        let tx_buf: [u8; 2] = [hi_byte, lo_byte];

        let mut rx_buf: [u8; 7] = [0; 7];
        self.i2c.write_read(self.address, &tx_buf, &mut rx_buf)?;

        let id: u8 = rx_buf[0];
        let mut x: u16 = rx_buf[1] as u16 + ((rx_buf[2] as u16) << 8);
        let mut y: u16 = rx_buf[3] as u16 + ((rx_buf[4] as u16) << 8);
        let size: u16 = rx_buf[5] as u16 + ((rx_buf[6] as u16) << 8);

        //println!("========== x = {:?}    y = {:?} ==========", x, y);

        match self.orientation {
            Orientation::Landscape => {
                // Don't need to do anything because x = x and y = y
            }
            Orientation::Portrait => {
                let temp: u16 = x;
                x = y;
                y = self.size.height - temp;
            }
            Orientation::InvertedLandscape => {
                x = self.size.width - x;
                y = self.size.height - y;
            }
            Orientation::InvertedPortrait => {
                let temp: u16 = x;
                x = self.size.width - y;
                y = temp;
            }
        }

        Ok(TouchPoint { id, x, y, size })
    }
}
