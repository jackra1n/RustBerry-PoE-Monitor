use crate::display_types::{Display, PCSENIOR8_STYLE, FONT_6X12, FONT_5X8, PROFONT12};
use linux_embedded_hal::I2cdev;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use display_interface::DisplayError;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text
};


pub struct PoeDisplay {
    display: Display
}

impl PoeDisplay {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let i2c = I2cdev::new("/dev/i2c-1")?;
        let display = initialize_display(i2c)?;
        Ok(PoeDisplay { display })
    }

    pub fn update(
        &mut self,
        ip_address: &str,
        cpu_usage: String,
        temp: String,
        ram_usage: String,
        disk_usage: &str,
    ) -> Result<(), DisplayError> {
        let disp = &mut self.display;

        let y_offset = 7;
        let display_width = 128;
        let char_width: i32 = 8;
        let x_margin = Point::new(2, 0).x_axis();
        let y_margin = Point::new(0, 1).y_axis();
    
        disp.clear(BinaryColor::Off)?;
    
        // top center: ip address
        let ip_width = ip_address.len() as i32 * char_width;
        let ip_x_position = (display_width - ip_width) / 2;
        Text::new(ip_address, Point::new(ip_x_position, y_offset), PCSENIOR8_STYLE).draw(disp)?;
    
        // middle left: cpu usage
        let cpu_width = cpu_usage.len() as i32 * char_width;
        let cpu_point = Point::new(34 - cpu_width, 12 + y_offset);
        let next = Text::new(&cpu_usage, cpu_point, PCSENIOR8_STYLE).draw(disp)?;
        let next = Text::new("%", next + y_margin, FONT_6X12).draw(disp)?;
        Text::new("CPU", next + x_margin, FONT_5X8).draw(disp)?;
        
        // bottom left: ram usage
        let ram_width = ram_usage.len() as i32 * char_width;
        let ram_point = Point::new(34 - ram_width, 23 + y_offset);
        let next = Text::new(&ram_usage, ram_point, PCSENIOR8_STYLE).draw(disp)?;
        let next = Text::new("%", next + y_margin, FONT_6X12).draw(disp)?;
        Text::new("RAM", next + x_margin, FONT_5X8).draw(disp)?;
    
        // middle right: temp
        let temp_width = temp.len() as i32 * char_width;
        let temp_point = Point::new(99 - temp_width, 12 + y_offset);
        let next = Text::new(&temp, temp_point, PCSENIOR8_STYLE).draw(disp)?;
        let next = Text::new("°", next + Point::new(0, 3), PROFONT12).draw(disp)?;
        Text::new("C", next - Point::new(0, 2), PCSENIOR8_STYLE).draw(disp)?;
    
        // bottom right: disk usage
        let disk_width = disk_usage.len() as i32 * char_width;
        let disk_point = Point::new(99 - disk_width, 23 + y_offset);
        let next = Text::new(disk_usage, disk_point, PCSENIOR8_STYLE).draw(disp)?;
        let next = Text::new("%", next + y_margin, FONT_6X12).draw(disp)?;
        Text::new("DISK", next + x_margin, FONT_5X8).draw(disp)?;
    
        disp.flush()?;
    
        Ok(())
    }
}

fn initialize_display(i2c: I2cdev) -> Result<Display, Box<dyn std::error::Error>> {
    let interface = I2CDisplayInterface::new(i2c);
    let mut disp = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    disp.init().map_err(|e| format!("Display initialization error: {:?}", e))?;
    Ok(disp)
}

