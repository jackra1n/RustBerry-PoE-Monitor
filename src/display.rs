use linux_embedded_hal::I2cdev;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306, mode::BufferedGraphicsMode};
use display_interface::DisplayError;
use embedded_graphics::{
    mono_font::{ascii, MonoTextStyleBuilder, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text
};
use profont::PROFONT_12_POINT;

const PROFONT12: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&PROFONT_12_POINT)
    .text_color(BinaryColor::On)
    .build();

const FONT_6X12: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&ascii::FONT_6X12)
    .text_color(BinaryColor::On)
    .build();

const FONT_5X8: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&ascii::FONT_5X8)
    .text_color(BinaryColor::On)
    .build();

type Display = Ssd1306<I2CInterface<I2cdev>, DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>;

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

        let y_offset = 8;
        let display_width = 128;
        let char_width: i32 = 8;
        let x_margin = Point::new(2, 0).x_axis();
    
        disp.clear(BinaryColor::Off)?;
    
        // top center: ip address
        let ip_width = ip_address.len() as i32 * char_width;
        let ip_x_position = (display_width - ip_width) / 2;
        Text::new(ip_address, Point::new(ip_x_position, y_offset), PROFONT12).draw(disp)?;
    
        // middle left: cpu usage
        let cpu_width = cpu_usage.len() as i32 * char_width;
        let cpu_point = Point::new(34 - cpu_width, 12 + y_offset);
        let next = Text::new(&cpu_usage, cpu_point, PROFONT12).draw(disp)?;
        let next = Text::new("%", next, FONT_6X12).draw(disp)?;
        Text::new("CPU", next + x_margin, FONT_5X8).draw(disp)?;
        
        // bottom left: ram usage
        let ram_width = ram_usage.len() as i32 * char_width;
        let ram_point = Point::new(34 - ram_width, 23 + y_offset);
        let next = Text::new(&ram_usage, ram_point, PROFONT12).draw(disp)?;
        let next = Text::new("%", next, FONT_6X12).draw(disp)?;
        Text::new("RAM", next + x_margin, FONT_5X8).draw(disp)?;
    
        // middle right: temp
        let temp_width = temp.len() as i32 * char_width;
        let temp_point = Point::new(99 - temp_width, 12 + y_offset);
        let next = Text::new(&temp, temp_point, PROFONT12).draw(disp)?;
        let next = Text::new("°", next, PROFONT12).draw(disp)?;
        Text::new("C", next, PROFONT12).draw(disp)?;
    
        // bottom right: disk usage
        let disk_width = disk_usage.len() as i32 * char_width;
        let disk_point = Point::new(99 - disk_width, 23 + y_offset);
        let next = Text::new(disk_usage, disk_point, PROFONT12).draw(disp)?;
        let next = Text::new("%", next, FONT_6X12).draw(disp)?;
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

