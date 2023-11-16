use linux_embedded_hal::I2cdev;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306, mode::BufferedGraphicsMode};
use embedded_graphics::{
    mono_font::{ascii, MonoTextStyleBuilder, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text
};
use profont::PROFONT_12_POINT;
use std::error::Error;
use std::fs;
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use machine_ip;


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

fn main() -> Result<(), Box<dyn Error>> {


    let mut disp = initialize_display()?;
    let mut sys: System = SystemExt::new_all();

    let mut last_disk_update = Instant::now();
    let disk_update_interval = Duration::from_secs(60);

    loop {
        sys.refresh_cpu();
        sys.refresh_memory();

        let ip_address = get_local_ip();
        let temp = format!("{:.1}", get_cpu_temperature());
        let cpu_usage = format!("{:.1}", get_cpu_usage(&sys));
        let ram_usage = format!("{:.1}", get_ram_usage(&sys));

        if last_disk_update.elapsed() >= disk_update_interval {
            sys.refresh_disks();
            last_disk_update = Instant::now();
        }
        let disk_usage = format!("{:.1}", get_disk_usage(&sys));

        update_display(&mut disp, &ip_address, cpu_usage, temp, ram_usage, disk_usage)?;

        thread::sleep(Duration::from_secs(1));
    }
}

fn initialize_display() -> Result<Ssd1306<I2CInterface<I2cdev>, DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>, Box<dyn std::error::Error>> {
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let interface = I2CDisplayInterface::new(i2c);
    let mut disp = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0).into_buffered_graphics_mode();

    disp.init().map_err(|e| format!("Display initialization error: {:?}", e))?;
    Ok(disp)
}

fn get_cpu_usage(sys: &System) -> f32 {
    let global_processor_info = sys.global_cpu_info();
    global_processor_info.cpu_usage()
}

fn get_cpu_temperature() -> f32 {
    let temp_contents = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp").unwrap();
    temp_contents.trim().parse::<f32>().unwrap() / 1000.0
}

fn get_ram_usage(sys: &System) -> f64 {
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    (used_memory as f64 / total_memory as f64) * 100.0
}

fn get_disk_usage(sys: &System) -> f64 {
    let disks = sys.disks();
    if let Some(disk) = disks.get(0) {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        if total_space > 0 {
            (1.0 - (available_space as f64 / total_space as f64)) * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    }
}

fn get_local_ip() -> String {
    match machine_ip::get() {
        Some(ip) => ip.to_string(),
        None => "No IP".to_string(),
    }
}

fn update_display(
    disp: &mut Ssd1306<I2CInterface<I2cdev>, DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>,
    ip_address: &str,
    cpu_usage: String,
    temp: String,
    ram_usage: String,
    disk_usage: String,
) -> Result<(), Box<dyn Error>> {
    let y_offset = 8;
    let display_width = 128;
    let char_width: i32 = 8;
    let x_margin = Point::new(2, 0).x_axis();

    disp.clear(BinaryColor::Off).unwrap();

    // top center: ip address
    let ip_width = ip_address.len() as i32 * char_width;
    let ip_x_position = (display_width - ip_width) / 2;
    Text::new(&ip_address, Point::new(ip_x_position, y_offset), PROFONT12).draw(disp).unwrap();

    // middle left: cpu usage
    let cpu_width = cpu_usage.len() as i32 * char_width;
    let next = Text::new(&cpu_usage, Point::new(34 - cpu_width, 12 + y_offset), PROFONT12).draw(disp).unwrap();
    let next = Text::new("%", next, FONT_6X12).draw(disp).unwrap();
    Text::new("CPU", next + x_margin, FONT_5X8).draw(disp).unwrap();
    
    // bottom left: ram usage
    let ram_width = ram_usage.len() as i32 * char_width;
    let next = Text::new(&ram_usage, Point::new(34 - ram_width, 23 + y_offset), PROFONT12).draw(disp).unwrap();
    let next = Text::new("%", next, FONT_6X12).draw(disp).unwrap();
    Text::new("RAM", next + x_margin, FONT_5X8).draw(disp).unwrap();

    // middle right: temp
    let temp_width = temp.len() as i32 * char_width;
    let next = Text::new(&temp, Point::new(99 - temp_width, 12 + y_offset), PROFONT12).draw(disp).unwrap();
    let next = Text::new("°", next, PROFONT12).draw(disp).unwrap();
    Text::new("C", next, PROFONT12).draw(disp).unwrap();

    // bottom right: disk usage
    let disk_width = disk_usage.len() as i32 * char_width;
    let next = Text::new(&disk_usage, Point::new(99 - disk_width, 23 + y_offset), PROFONT12).draw(disp).unwrap();
    let next = Text::new("%", next, FONT_6X12).draw(disp).unwrap();
    Text::new("DISK", next + x_margin, FONT_5X8).draw(disp).unwrap();

    disp.flush().unwrap();

    Ok(())
}
