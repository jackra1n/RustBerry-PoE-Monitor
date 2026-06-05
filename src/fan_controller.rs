use linux_embedded_hal::I2cdev;
use log::debug;
use pcf857x::{Pcf8574, SlaveAddr};

const I2C_BUS_PATH: &str = "/dev/i2c-1";

pub struct FanController {
    expander: Pcf8574<I2cdev>,
    pub is_running: bool,
    pub temp_on: f32,
    pub temp_off: f32,
}

impl FanController {
    pub fn new(temp_on: f32, temp_off: f32) -> Result<Self, Box<dyn std::error::Error>> {
        debug!("Initializing FanController");
        if temp_off <= 0.0 || temp_on <= 0.0 {
            return Err("Temperatures must be greater than 0".into());
        }
        if temp_on <= temp_off {
            return Err("temp_on must be greater than temp_off".into());
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
        self.expander
            .set(0xFE)
            .map_err(|e| format!("Fan on error: {:?}", e))?;
        self.is_running = true;
        Ok(())
    }

    pub fn fan_off(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Sending fan off signal [p0: high]");
        self.expander
            .set(0xFF)
            .map_err(|e| format!("Fan off error: {:?}", e))?;
        self.is_running = false;
        Ok(())
    }
}
