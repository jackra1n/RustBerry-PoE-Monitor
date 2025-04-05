use embedded_graphics::prelude::Point;
use env_logger::{Builder, Env};
use log::{debug, info, trace};
use ssd1306::prelude::Brightness;
use std::error::Error;
use std::fs;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

mod fan_controller;
use fan_controller::FanController;

mod config;
use config::Config;

mod display;
use display::PoeDisplay;

mod display_types;

fn main() -> Result<(), Box<dyn Error>> {
    let env = Env::default().default_filter_or("info");
    Builder::from_env(env).init();

    let config = Config::load()?;

    let version = env!("CARGO_PKG_VERSION");

    debug!("Binary info:");
    debug!("================================");
    debug!("rustberry-poe-monitor:   {}", version);
    debug!("Target OS:               {}", std::env::consts::OS);
    debug!("Target Family:           {}", std::env::consts::FAMILY);
    debug!("Target Architecture:     {}", std::env::consts::ARCH);
    debug!("Config loaded: {:?}", config);

    let mut poe_disp = PoeDisplay::new(&config.display)?;

    let mut fan_controller = FanController::new(config.fan.temp_on, config.fan.temp_off)?;
    info!(
        "Fan controller initialized. temp-on: {}, temp-off: {}",
        fan_controller.temp_on, fan_controller.temp_off
    );

    let mut sys: System = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
            .with_memory(MemoryRefreshKind::nothing().with_ram()),
    );

    debug!("System initialized. System info:");
    debug!("================================");
    debug!(
        "System name:             {}",
        System::name().unwrap_or_default()
    );
    debug!(
        "System kernel version:   {}",
        System::kernel_version().unwrap_or_default()
    );
    debug!(
        "System OS version:       {}",
        System::os_version().unwrap_or_default()
    );

    let mut disk_usage = String::new();
    info!("Starting main loop");

    fan_controller.fan_off()?;

    let mut ip_address_str = get_ip_address();
    let mut iteration = 0;

    let mut shift_offset = Point::new(0, 0);
    let mut last_shift_time = Instant::now();
    let shift_pattern = [
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(1, 1),
        Point::new(0, 1),
    ];
    let mut shift_index = 0;
    let shift_interval = Duration::from_secs(60);

    let start_time = Instant::now();
    let screen_timeout_duration = config.display_timeout();
    let mut screen_dimmed = false;

    loop {
        let elapsed_time = start_time.elapsed();

        if screen_timeout_duration.as_secs() > 0
            && !screen_dimmed
            && elapsed_time >= screen_timeout_duration
        {
            info!("Screen timeout reached. Dimming display.");
            poe_disp
                .set_brightness(Brightness::DIMMEST)
                .map_err(|e| format!("Failed to dim display: {:?}", e))?;
            screen_dimmed = true;
        }

        if last_shift_time.elapsed() >= shift_interval {
            shift_index = (shift_index + 1) % shift_pattern.len();
            shift_offset = shift_pattern[shift_index];
            last_shift_time = Instant::now();
            debug!("Shifting display pixels to offset: {:?}", shift_offset);
        }

        sys.refresh_cpu_usage();
        sys.refresh_memory();

        if iteration % 10 == 0 {
            ip_address_str = get_ip_address();
        }

        let cpu_temp = get_cpu_temperature();
        let cpu_temp_str = format!("{:.1}", cpu_temp);

        let cpu_usage = format!("{:.1}", sys.global_cpu_usage());
        let ram_usage = format!("{:.1}", get_ram_usage(&sys));

        trace!(
            "Checking fan controller. Fan running: {}",
            fan_controller.is_running
        );
        trace!("CPU Temp: {}", cpu_temp);

        if fan_controller.is_running {
            if cpu_temp <= fan_controller.temp_off {
                fan_controller.fan_off()?;
            }
        } else if cpu_temp >= fan_controller.temp_on {
            fan_controller.fan_on()?;
        }

        if iteration % 60 == 0 {
            disk_usage = format!("{:.1}", get_disk_usage());
        }

        poe_disp
            .update(
                &ip_address_str,
                cpu_usage,
                cpu_temp_str,
                ram_usage,
                &disk_usage,
                shift_offset,
            )
            .map_err(|e| format!("Display update error: {:?}", e))?;

        iteration += 1;
        thread::sleep(Duration::from_secs(1));
    }
}

fn get_ip_address() -> String {
    Command::new("hostname")
        .arg("-I")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .and_then(|s| s.split_whitespace().next().map(str::to_string))
            } else {
                None
            }
        })
        .unwrap_or_else(|| "0.0.0.0".to_string())
        .trim()
        .to_string()
}

fn get_cpu_temperature() -> f32 {
    match fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        Ok(contents) => contents.trim().parse::<f32>().unwrap_or(0.0) / 1000.0,
        Err(e) => {
            log::warn!("Failed to read CPU temperature: {}", e);
            0.0
        }
    }
}

fn get_ram_usage(sys: &System) -> f64 {
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    (used_memory as f64 / total_memory as f64) * 100.0
}

fn get_disk_usage() -> f64 {
    let mut disks = Disks::new_with_refreshed_list();
    if let Some(disk) = disks.first_mut() {
        disk.refresh();
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
