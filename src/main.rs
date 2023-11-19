use std::error::Error;
use std::fs;
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt, CpuExt, DiskExt};

mod fan_controller;
use fan_controller::FanController;

mod display;
use display::PoeDisplay;


fn main() -> Result<(), Box<dyn Error>> {
    let mut poe_disp = PoeDisplay::new()?;
    let mut fan_controller = FanController::new(50.0, 60.0)?;

    let mut sys: System = SystemExt::new_all();

    let mut disk_usage = String::new();
    let disk_update_interval = Duration::from_secs(60);
    let mut last_disk_update = Instant::now() - disk_update_interval;

    loop {
        sys.refresh_cpu();
        sys.refresh_memory();

        let ip_address = get_local_ip();
        let temp = get_cpu_temperature();
        let temp_str = format!("{:.1}", temp);
        let cpu_usage = format!("{:.1}", get_cpu_usage(&sys));
        let ram_usage = format!("{:.1}", get_ram_usage(&sys));


        if fan_controller.is_running {
            if temp <= fan_controller.temp_off {
                fan_controller.fan_off()?;
            }
        } else if temp >= fan_controller.temp_on {
            fan_controller.fan_on()?;
        }

        if last_disk_update.elapsed() >= disk_update_interval {
            sys.refresh_disks();
            last_disk_update = Instant::now();
            disk_usage = format!("{:.1}", get_disk_usage(&sys));
        }

        poe_disp.update(&ip_address, cpu_usage, temp_str, ram_usage, &disk_usage).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
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
