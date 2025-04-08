use crate::config::DisplayConfig as AppDisplayConfig;
use crate::display_types::{Display, FONT_5X8, FONT_6X12, PCSENIOR8_STYLE, PROFONT12};
use display_interface::DisplayError;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, text::Text};
use linux_embedded_hal::I2cdev;
use log::{info, warn};

use ssd1306::mode::DisplayConfig;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

pub struct PoeDisplay {
    display: Display,
}

impl PoeDisplay {
    pub fn new(display_config: &AppDisplayConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let i2c = I2cdev::new("/dev/i2c-1")?;
        let display = initialize_display(i2c, display_config)?;
        Ok(PoeDisplay { display })
    }

    pub fn set_brightness(&mut self, brightness: Brightness) -> Result<(), DisplayError> {
        self.display.set_brightness(brightness)
    }

    pub fn update(
        &mut self,
        ip_address: &str,
        cpu_usage: String,
        temp: String,
        ram_usage: String,
        disk_usage: &str,
        offset: Point,
    ) -> Result<(), DisplayError> {
        let disp = &mut self.display;

        let y_offset = 7;
        let display_width = 128;
        let char_width: i32 = 8;

        let x_margin = Point::new(2, 0);

        disp.clear(BinaryColor::Off)?;

        let ip_width = ip_address.len() as i32 * char_width;
        let ip_x_position = (display_width - ip_width) / 2;
        let ip_pos = Point::new(ip_x_position, y_offset) + offset;
        Text::new(ip_address, ip_pos, PCSENIOR8_STYLE).draw(disp)?;

        let cpu_width = cpu_usage.len() as i32 * char_width;
        let cpu_pos = Point::new(34 - cpu_width, 12 + y_offset) + offset;
        let next = Text::new(&cpu_usage, cpu_pos, PCSENIOR8_STYLE).draw(disp)?;
        let next = Text::new("%", next, FONT_6X12).draw(disp)?;
        Text::new("CPU", next + x_margin, FONT_5X8).draw(disp)?;

        let ram_width = ram_usage.len() as i32 * char_width;
        let ram_pos = Point::new(34 - ram_width, 23 + y_offset) + offset;
        let next = Text::new(&ram_usage, ram_pos, PCSENIOR8_STYLE).draw(disp)?;
        let next = Text::new("%", next, FONT_6X12).draw(disp)?;
        Text::new("RAM", next + x_margin, FONT_5X8).draw(disp)?;

        let temp_width = temp.len() as i32 * char_width;
        let temp_pos = Point::new(99 - temp_width, 12 + y_offset) + offset;
        let next = Text::new(&temp, temp_pos, PCSENIOR8_STYLE).draw(disp)?;
        let next = Text::new("°", next + Point::new(0, 3), PROFONT12).draw(disp)?;
        Text::new("C", next - Point::new(0, 2), PCSENIOR8_STYLE).draw(disp)?;

        let disk_width = disk_usage.len() as i32 * char_width;
        let disk_pos = Point::new(99 - disk_width, 23 + y_offset) + offset;
        let next = Text::new(disk_usage, disk_pos, PCSENIOR8_STYLE).draw(disp)?;
        let next = Text::new("%", next, FONT_6X12).draw(disp)?;
        Text::new("DISK", next + x_margin, FONT_5X8).draw(disp)?;

        disp.flush()
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

    let initial_brightness = match display_config.brightness {
        0 => Brightness::DIMMEST,
        1 => Brightness::DIM,
        2 => Brightness::NORMAL,
        3 => Brightness::BRIGHT,
        4 => Brightness::BRIGHTEST,
        _ => {
            warn!(
                "Invalid brightness value {} in config, defaulting to DIMMEST",
                display_config.brightness
            );
            Brightness::DIMMEST
        }
    };

    disp.set_brightness(initial_brightness)
        .map_err(|e| format!("Failed to set initial brightness: {:?}", e))?;

    info!(
        "Display initialized with brightness: {:?}",
        initial_brightness
    );

    Ok(disp)
}
