use linux_embedded_hal::{I2cdev, i2cdev::core::I2CDevice};
use std::io::Error;

pub struct FanController {
    i2c: I2cdev,
    address: u8,
    temperature_on: f32,
    temperature_off: f32,
}

impl FanController {
    pub fn new(temperature_on: f32, temperature_off: f32) -> Result<Self, std::io::Error> {
        let i2c = I2cdev::new("/dev/i2c-1")?;
        Ok(FanController {
            i2c,
            address: 0x20,
            temperature_on: 60.0,
            temperature_off: 50.0,
        })
    }

    pub fn fan_on(&mut self) -> Result<(), Error> {
        let current_value = self.i2c.smbus_read_byte()?;
        self.i2c.smbus_write_byte(0xFE & current_value)?;
        Ok(())
    }

    pub fn fan_off(&mut self) -> Result<(), Error> {
        let current_value = self.i2c.smbus_read_byte()?;
        self.i2c.smbus_write_byte(0x01 | current_value)?;
        Ok(())
    }
}