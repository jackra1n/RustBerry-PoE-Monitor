use linux_embedded_hal::I2cdev;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306, mode::BufferedGraphicsMode};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X12, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use std::error::Error;
use std::fs;
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use machine_ip;

fn main() -> Result<(), Box<dyn Error>> {
    let mut disp = initialize_display()?;
    let mut sys: System = SystemExt::new_all();

    let mut last_disk_update = Instant::now();
    let disk_update_interval = Duration::from_secs(60);

    loop {
        sys.refresh_cpu();
        sys.refresh_memory();

        let temp = get_cpu_temperature();
        let ip_address = get_local_ip();
        let cpu_usage = get_cpu_usage(&sys);
        let ram_usage = get_ram_usage(&sys);

        if last_disk_update.elapsed() >= disk_update_interval {
            sys.refresh_disks();
            last_disk_update = Instant::now();
        }
        let disk_usage = get_disk_usage(&sys);

        update_display(&mut disp, &ip_address, &cpu_usage, temp, ram_usage, disk_usage)?;

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
    cpu_usage: &f32,
    temp: f32,
    ram_usage: f64,
    disk_usage: f64,
) -> Result<(), Box<dyn Error>> {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X12)
        .text_color(BinaryColor::On)
        .build();

    disp.clear(BinaryColor::Off).unwrap();

    let offset = 8;

    let ip_text = Text::new(&ip_address, Point::new(0, 0 + offset), text_style);
    ip_text.draw(disp).unwrap();

    let cpu_usage_string = format!("{:.1}%CPU", cpu_usage);
    let cpu_usage_text = Text::new(&cpu_usage_string, Point::new(0, 11 + offset), text_style);
    cpu_usage_text.draw(disp).unwrap();

    let temp_string = format!("{:.1}°C", temp);
    let temp_text = Text::new(&temp_string, Point::new(64, 11 + offset), text_style);
    temp_text.draw(disp).unwrap();

    let ram_usage_string = format!("{:.1}%RAM", ram_usage);
    let ram_usage_text = Text::new(&ram_usage_string, Point::new(0, 22 + offset), text_style);
    ram_usage_text.draw(disp).unwrap();

    let disk_usage_string = format!("{:.1}%DISK", disk_usage);
    let disk_usage_text = Text::new(&disk_usage_string, Point::new(64, 22 + offset), text_style);
    disk_usage_text.draw(disp).unwrap();

    disp.flush().unwrap();
    Ok(())
}
