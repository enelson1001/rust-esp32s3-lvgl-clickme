/// A minimal implementation of the GT911 to work with Lvgl since Lvgl only uses a single touch point
/// The default orientation and size are based on the aliexpress ESP 7 inch capactive touch development
/// board model ESP-8048S070C
use embedded_hal::i2c::{I2c, SevenBitAddress};

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

/// Current state of the driver
#[derive(Copy, Clone, Debug)]
pub enum TouchState {
    PRESSED(TouchPoint),
    RELEASED(TouchPoint),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TouchPoint {
    pub x: u16,
    pub y: u16,
}

/// Driver representation holding:
///
/// - The I2C Slave address of the GT911
/// - The I2C Bus used to communicate with the GT911
/// - The screen/panel orientation
/// - The scree/panel dimesions

pub struct GT911<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    address: u8,
    i2c: I2C,
    orientation: Orientation,
    size: Dimension,
    last_tp: TouchPoint,
}

impl<I2C> GT911<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    pub fn new(i2c: I2C) -> Self {
        Self {
            address: DEFAULT_GT911_ADDRESS,
            i2c,
            orientation: Orientation::Landscape,
            size: Dimension {
                height: 480,
                width: 800,
            },
            last_tp: TouchPoint { x: 0, y: 0 },
        }
    }

    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    pub fn set_size(&mut self, height: u16, width: u16) {
        self.size = Dimension { height, width };
    }

    // Useful function to determine if you are communicating with GT911, The GT911 must first be reset.
    // The return string should be - 911
    pub fn read_product_id(&mut self) -> Result<String, I2C::Error> {
        let mut rx_buf: [u8; 4] = [0; 4];

        let product_id_reg: u16 = Reg::ProductId as u16;

        let hi_byte: u8 = (product_id_reg >> 8).try_into().unwrap();
        let lo_byte: u8 = (product_id_reg & 0xFF).try_into().unwrap();
        let tx_buf: [u8; 2] = [hi_byte, lo_byte];

        self.i2c.write_read(self.address, &tx_buf, &mut rx_buf)?;

        Ok(std::str::from_utf8(&rx_buf).unwrap().to_string())
    }

    pub fn clear_point_info_reg(&mut self) -> Result<(), I2C::Error> {
        let point_info_reg: u16 = Reg::PointInfo as u16;
        let hi_byte: u8 = (point_info_reg >> 8).try_into().unwrap();
        let lo_byte: u8 = (point_info_reg & 0xFF).try_into().unwrap();
        let tx_buf: [u8; 3] = [hi_byte, lo_byte, 0u8];

        self.i2c.write(self.address, &tx_buf)?;

        Ok(())
    }

    pub fn read_touch(&mut self) -> Result<TouchState, I2C::Error> {
        let mut rx_buf: [u8; 1] = [0xFF];

        let point_info_reg: u16 = Reg::PointInfo as u16;
        let hi_byte: u8 = (point_info_reg >> 8).try_into().unwrap();
        let lo_byte: u8 = (point_info_reg & 0xFF).try_into().unwrap();
        let tx_buf: [u8; 2] = [hi_byte, lo_byte];

        // Read point info register 0x814E
        self.i2c.write_read(self.address, &tx_buf, &mut rx_buf)?;

        // Reset point_info register after reading it
        let tx_buf: [u8; 3] = [hi_byte, lo_byte, 0u8];
        self.i2c.write(self.address, &tx_buf)?;

        let point_info = rx_buf[0];
        let status = point_info & 0x80;

        // Number of detected touch points
        let touch_pt_count = point_info & 0x07;
        let mut touch_state = TouchState::RELEASED(self.last_tp);

        // If status == 0 (no touch)
        if status != 0 {
            let tp = self.read_touch_point(Reg::Point1 as u16).unwrap();
            // If touchpoint != 1 (multiple touches) then return touchstate as RELEASED with last touchpoint coordinates,
            // otherwise return touchstate as PRESSED along with the current touchpoint coordinates.
            if touch_pt_count != 1 {
                touch_state = TouchState::RELEASED(self.last_tp);
            } else {
                touch_state = TouchState::PRESSED(tp);
                self.last_tp = tp;
            }
        }

        Ok(touch_state)
    }

    fn read_touch_point(&mut self, point_register: u16) -> Result<TouchPoint, I2C::Error> {
        let hi_byte: u8 = (point_register >> 8).try_into().unwrap();
        let lo_byte: u8 = (point_register & 0xFF).try_into().unwrap();
        let tx_buf: [u8; 2] = [hi_byte, lo_byte];

        let mut rx_buf: [u8; 7] = [0; 7];
        self.i2c.write_read(self.address, &tx_buf, &mut rx_buf)?;

        let mut x: u16 = rx_buf[1] as u16 + ((rx_buf[2] as u16) << 8);
        let mut y: u16 = rx_buf[3] as u16 + ((rx_buf[4] as u16) << 8);

        //println!("========== x = {:?}    y = {:?} ==========", x, y);

        match self.orientation {
            Orientation::Landscape => {
                //x = x;
                //y = y;
                // x = x, y = y
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

        Ok(TouchPoint { x, y })
    }
}
