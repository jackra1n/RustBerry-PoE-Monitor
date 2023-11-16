use linux_embedded_hal::I2cdev;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306, mode::BufferedGraphicsMode};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use std::error::Error;
use std::fs;
use std::thread;
use std::time::Duration;
use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use machine_ip;

fn main() -> Result<(), Box<dyn Error>> {
    let mut disp = initialize_display()?;
    let mut sys: System = SystemExt::new_all();

    loop {
        sys.refresh_all();

        let temp = get_cpu_temperature()?;
        let ip_address = get_local_ip();
        let cpu_info = get_cpu_info(&sys);
        let ram_usage = get_ram_usage(&sys);
        let disk_usage = get_disk_usage(&sys);

        update_display(&mut disp, &ip_address, &cpu_info, temp, ram_usage, disk_usage)?;

        thread::sleep(Duration::from_secs(1));
    }
}

fn initialize_display() -> Result<Ssd1306<I2CInterface<I2cdev>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>, Box<dyn std::error::Error>> {
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let interface = I2CDisplayInterface::new(i2c);
    let mut disp = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_buffered_graphics_mode();

    disp.init().map_err(|e| format!("Display initialization error: {:?}", e))?;
    Ok(disp)
}

fn get_cpu_info(sys: &System) -> String {
    let global_processor_info = sys.global_cpu_info();
    let cpu_usage = global_processor_info.cpu_usage();
    format!("{:.1}%", cpu_usage)
}

fn get_cpu_temperature() -> Result<f32, Box<dyn Error>> {
    let temp_contents = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?;
    let temp_celsius = temp_contents.trim().parse::<f32>()? / 1000.0;
    Ok(temp_celsius)
}

fn get_ram_usage(sys: &System) -> String {
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    format!("{:.1}%", (used_memory as f64 / total_memory as f64) * 100.0)
}

fn get_disk_usage(sys: &System) -> String {
    let disks = sys.disks();
    if let Some(disk) = disks.get(0) {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        if total_space > 0 {
            format!("{:.1}%", (1.0 - (available_space as f64 / total_space as f64)) * 100.0)
        } else {
            "N/A".to_string()
        }
    } else {
        "Disk Not Found".to_string()
    }
}

fn get_local_ip() -> String {
    match machine_ip::get() {
        Some(ip) => ip.to_string(),
        None => "No IP".to_string(),
    }
}

fn update_display(
    disp: &mut Ssd1306<I2CInterface<I2cdev>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
    ip_address: &str,
    cpu_info: &str,
    temp: f32,
    ram_usage: String,
    disk_usage: String,
) -> Result<(), Box<dyn Error>> {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X9)
        .text_color(BinaryColor::On)
        .build();

    let info = format!(
        "{}\n{}%CPU {}°C\n{}%RAM\n{}%DISK",
        ip_address,
        cpu_info,
        temp,
        ram_usage,
        disk_usage
    );
    Text::new(&info, Point::new(0, 0), text_style).draw(disp).unwrap();

    disp.flush().unwrap();
    Ok(())
}
