use std::thread;
use std::time::{Duration, SystemTime};
use chrono::Local;

use rppal::i2c::I2c;
use ssd1306::{prelude::*, Builder};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(0x3C)?;

    let interface = I2cInterface::new(i2c);
    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();
    disp.init()?;

    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    loop {
        let now = Local::now();
        let time = now.format("%H:%M:%S").to_string();

        disp.clear();
        Text::new(&time, Point::new(0, 32), text_style)
            .draw(&mut disp)?;

        disp.flush()?;
        
        thread::sleep(Duration::from_secs(1));
    }
}

