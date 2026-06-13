use crate::config::DisplayConfig as AppDisplayConfig;
use crate::display_types::{Display, FONT_5X8, FONT_6X12, PCSENIOR8, PCSENIOR8_STYLE, PROFONT12};
use display_interface::DisplayError;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, text::Text};
use linux_embedded_hal::I2cdev;
use log::{debug, info, warn};

use ssd1306::mode::DisplayConfig;
use ssd1306::prelude::Brightness;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

const DISPLAY_WIDTH: u32 = 128;
const IP_ROW_Y: i32 = 7;
const STATS_ROW1_Y: i32 = 19;
const STATS_ROW2_Y: i32 = 30;
const LEFT_COL_RIGHT: i32 = 34;
const RIGHT_COL_RIGHT: i32 = 99;
const X_MARGIN: Point = Point::new(2, 0);
const VALUE_CHAR_WIDTH: i32 = PCSENIOR8.character_size.width as i32;

pub struct PoeDisplay {
    display: Display,
}

impl PoeDisplay {
    pub fn new(display_config: &AppDisplayConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let i2c = I2cdev::new("/dev/i2c-1")?;
        let display = initialize_display(i2c, display_config)?;
        Ok(PoeDisplay { display })
    }

    pub fn set_brightness(&mut self, brightness: Brightness) -> Result<(), Box<dyn std::error::Error>> {
        self.display.set_brightness(brightness).map_err(draw_err)?;
        Ok(())
    }

    pub fn display_off(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Turning display OFF.");
        self.display.set_display_on(false).map_err(draw_err)?;
        Ok(())
    }

    pub fn display_on(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Turning display ON.");
        self.display.set_display_on(true).map_err(draw_err)?;
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
        let disp = &mut self.display;

        disp.clear(BinaryColor::Off).map_err(draw_err)?;

        let ip_width = ip_address.len() as i32 * VALUE_CHAR_WIDTH;
        let ip_x_position = (DISPLAY_WIDTH as i32 - ip_width) / 2;
        let ip_pos = Point::new(ip_x_position, IP_ROW_Y) + offset;
        Text::new(ip_address, ip_pos, PCSENIOR8_STYLE)
            .draw(disp)
            .map_err(draw_err)?;

        let cpu_width = cpu_usage.len() as i32 * VALUE_CHAR_WIDTH;
        let cpu_pos = Point::new(LEFT_COL_RIGHT - cpu_width, STATS_ROW1_Y) + offset;
        let next = Text::new(cpu_usage, cpu_pos, PCSENIOR8_STYLE)
            .draw(disp)
            .map_err(draw_err)?;
        let next = Text::new("%", next, FONT_6X12).draw(disp).map_err(draw_err)?;
        Text::new("CPU", next + X_MARGIN, FONT_5X8)
            .draw(disp)
            .map_err(draw_err)?;

        let ram_width = ram_usage.len() as i32 * VALUE_CHAR_WIDTH;
        let ram_pos = Point::new(LEFT_COL_RIGHT - ram_width, STATS_ROW2_Y) + offset;
        let next = Text::new(ram_usage, ram_pos, PCSENIOR8_STYLE)
            .draw(disp)
            .map_err(draw_err)?;
        let next = Text::new("%", next, FONT_6X12).draw(disp).map_err(draw_err)?;
        Text::new("RAM", next + X_MARGIN, FONT_5X8)
            .draw(disp)
            .map_err(draw_err)?;

        let temp_width = temp.len() as i32 * VALUE_CHAR_WIDTH;
        let temp_pos = Point::new(RIGHT_COL_RIGHT - temp_width, STATS_ROW1_Y) + offset;
        let next = Text::new(temp, temp_pos, PCSENIOR8_STYLE)
            .draw(disp)
            .map_err(draw_err)?;
        let next = Text::new("°", next + Point::new(0, 3), PROFONT12)
            .draw(disp)
            .map_err(draw_err)?;
        Text::new("C", next - Point::new(0, 2), PCSENIOR8_STYLE)
            .draw(disp)
            .map_err(draw_err)?;

        let disk_width = disk_usage.len() as i32 * VALUE_CHAR_WIDTH;
        let disk_pos = Point::new(RIGHT_COL_RIGHT - disk_width, STATS_ROW2_Y) + offset;
        let next = Text::new(disk_usage, disk_pos, PCSENIOR8_STYLE)
            .draw(disp)
            .map_err(draw_err)?;
        let next = Text::new("%", next, FONT_6X12).draw(disp).map_err(draw_err)?;
        Text::new("DISK", next + X_MARGIN, FONT_5X8)
            .draw(disp)
            .map_err(draw_err)?;

        disp.flush().map_err(draw_err)?;
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

fn draw_err(e: DisplayError) -> Box<dyn std::error::Error> {
    format!("Display draw error: {:?}", e).into()
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
