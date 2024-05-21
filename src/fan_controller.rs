use linux_embedded_hal::{I2cdev, i2cdev::core::I2CDevice};
use anyhow::{anyhow, Result};
use log::debug;

const SLAVE_ADDRESS: u16 = 0x20;
const FAN_ON_COMMAND: u8 = 0xFE;
const FAN_OFF_COMMAND: u8 = 0x01;

pub struct FanController {
    i2c: I2cdev,
    pub is_running: bool,
    pub temp_on: f32,
    pub temp_off: f32,
}

impl FanController {
    pub fn new(temp_on: f32, temp_off: f32) -> Result<Self> {
        debug!("Initializing FanController");
        if temp_off <= 0.0 || temp_on <= 0.0 {
            return Err(anyhow!("Temperatures must be greater than 0"));
        }
        if temp_on <= temp_off {
            return Err(anyhow!("temp_on must be greater than temp_off"));
        }

        let i2c = I2cdev::new("/dev/i2c-1")?;
        debug!("I2C initialized");
        
        Ok(FanController {
            i2c,
            is_running: false,
            temp_off,
            temp_on,
        })
    }

    pub fn fan_on(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Sending fan on command");
        self.i2c.set_slave_address(SLAVE_ADDRESS)?;
        self.i2c.smbus_write_byte(FAN_ON_COMMAND)?;
        self.is_running = true;
        Ok(())
    }

    pub fn fan_off(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Sending fan off command");
        self.i2c.set_slave_address(SLAVE_ADDRESS)?;
        self.i2c.smbus_write_byte(FAN_OFF_COMMAND)?;
        self.is_running = false;
        Ok(())
    }
}