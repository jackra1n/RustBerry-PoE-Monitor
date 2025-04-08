use anyhow::{anyhow, Result};
use linux_embedded_hal::I2cdev;
use log::debug;
use pcf857x::OutputPin;
use pcf857x::{Pcf8574, SlaveAddr};

const I2C_BUS_PATH: &str = "/dev/i2c-1";

pub struct FanController {
    expander: Pcf8574<I2cdev>,
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

        let i2c = I2cdev::new(I2C_BUS_PATH)?;
        let address = SlaveAddr::default();
        let expander = Pcf8574::new(i2c, address);
        debug!("pcf8574 IO Expander initialized");

        Ok(FanController {
            expander,
            is_running: false,
            temp_off,
            temp_on,
        })
    }

    pub fn fan_on(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Sending fan on signal [p0: low]");
        let mut parts = self.expander.split();
        parts.p0.set_low().unwrap();
        self.is_running = true;
        Ok(())
    }

    pub fn fan_off(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Sending fan off signal [p0: high]");
        let mut parts = self.expander.split();
        parts.p0.set_high().unwrap();
        self.is_running = false;
        Ok(())
    }
}
