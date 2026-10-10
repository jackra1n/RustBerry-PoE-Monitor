use crate::config::DisplayConfig as AppDisplayConfig;
use embedded_graphics::prelude::Point;
use linux_embedded_hal::I2cdev;
use log::{debug, info, warn};
use rustberry_poe_monitor::render;

use ssd1306::mode::DisplayConfig;
use ssd1306::prelude::Brightness;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

type Display =
    Ssd1306<I2CInterface<I2cdev>, DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>;

pub struct PoeDisplay {
    display: Display,
    last_frame: String,
}

impl PoeDisplay {
    pub fn new(display_config: &AppDisplayConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let i2c = I2cdev::new("/dev/i2c-1")?;
        let display = initialize_display(i2c, display_config)?;
        Ok(PoeDisplay {
            display,
            last_frame: String::new(),
        })
    }

    pub fn set_brightness(
        &mut self,
        brightness: Brightness,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.display
            .set_brightness(brightness)
            .map_err(|e| format!("Failed to set brightness: {:?}", e))?;
        Ok(())
    }

    pub fn display_off(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Turning display OFF.");
        self.display
            .set_display_on(false)
            .map_err(|e| format!("Failed to turn display off: {:?}", e))?;
        Ok(())
    }

    pub fn display_on(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Turning display ON.");
        self.display
            .set_display_on(true)
            .map_err(|e| format!("Failed to turn display on: {:?}", e))?;
        Ok(())
    }

    pub fn update(
        &mut self,
        ip_address: &str,
        cpu_usage: &str,
        temp: &str,
        ram_usage: &str,
        disk_usage: &str,
        offset: Point,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let frame = format!("{ip_address} {cpu_usage} {temp} {ram_usage} {disk_usage} {offset:?}");
        if frame == self.last_frame {
            return Ok(());
        }
        render::draw(
            &mut self.display,
            ip_address,
            cpu_usage,
            temp,
            ram_usage,
            disk_usage,
            offset,
        )
        .and_then(|()| self.display.flush())
        .map_err(|e| format!("Failed to update display: {:?}", e))?;
        self.last_frame = frame;
        Ok(())
    }
}

fn map_brightness_value(value: u8) -> Brightness {
    match value {
        0 => Brightness::DIMMEST,
        1 => Brightness::DIM,
        2 => Brightness::NORMAL,
        3 => Brightness::BRIGHT,
        4 => Brightness::BRIGHTEST,
        _ => {
            warn!(
                "Invalid brightness value {} encountered, defaulting to DIMMEST",
                value
            );
            Brightness::DIMMEST
        }
    }
}

fn initialize_display(
    i2c: I2cdev,
    display_config: &AppDisplayConfig,
) -> Result<Display, Box<dyn std::error::Error>> {
    let interface = I2CDisplayInterface::new(i2c);
    let mut disp = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    disp.init()
        .map_err(|e| format!("Display initialization error: {:?}", e))?;

    let initial_brightness = map_brightness_value(display_config.brightness);

    disp.set_brightness(initial_brightness)
        .map_err(|e| format!("Failed to set initial brightness: {:?}", e))?;

    info!(
        "Display initialized with brightness: {:?}",
        initial_brightness
    );

    Ok(disp)
}
