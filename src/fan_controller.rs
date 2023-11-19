use linux_embedded_hal::{I2cdev, i2cdev::core::I2CDevice};


pub struct FanController {
    i2c: I2cdev,
    temperature_off: f32,
    temperature_on: f32,
}

impl FanController {
    pub fn new(temperature_off: f32, temperature_on: f32) -> Result<Self, std::io::Error> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        Ok(FanController {
            i2c,
            temperature_off: 50.0,
            temperature_on: 60.0,
        })
    }

    pub fn fan_on(&mut self) {
        self.i2c.set_slave_address(0x20);
        match self.i2c.smbus_write_byte(0xFE) {
            Ok(_) => println!("Fan on"),
            Err(error) => println!("Fan on error: {}", error),
        }
    }

    pub fn fan_off(&mut self) {
        self.i2c.set_slave_address(0x20);
        match self.i2c.smbus_write_byte(0x01) {
            Ok(_) => println!("Fan on"),
            Err(error) => println!("Fan on error: {}", error),
        }
    }

    pub fn update(&mut self, temperature: f32) {
        if temperature >= self.temperature_on {
            self.fan_on();
        } else if temperature <= self.temperature_off {
            self.fan_off();
        }
    }
}