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

struct AppState {
    last_shift_time: Instant,
    shift_index: usize,
    shift_offset: Point,
    last_periodic_toggle_time: Instant,
    is_display_periodically_on: bool,
    screen_dimmed: bool,
}

struct SystemStats {
    ip_address: String,
    cpu_usage: String,
    cpu_temp: f32,
    cpu_temp_str: String,
    ram_usage: String,
    disk_usage: String,
}

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

    info!("Starting main loop");

    fan_controller.fan_off()?;

    let shift_interval = Duration::from_secs(60);
    let shift_pattern = [Point::new(0, 0), Point::new(1, 0)];

    let mut app_state = AppState {
        last_shift_time: Instant::now(),
        shift_index: 0,
        shift_offset: Point::new(0, 0),
        last_periodic_toggle_time: Instant::now(),
        is_display_periodically_on: true,
        screen_dimmed: false,
    };

    let start_time = Instant::now();

    loop {
        let now = Instant::now();

        update_display_state(
            &config,
            start_time,
            now,
            config.display_timeout(),
            config.periodic_on_duration(),
            config.periodic_off_duration(),
            &mut app_state,
            &mut poe_disp,
        )?;

        update_pixel_shift(now, shift_interval, &shift_pattern, &mut app_state);

        let stats = gather_stats(&mut sys);

        handle_fan_control(&mut fan_controller, stats.cpu_temp)?;

        if app_state.is_display_periodically_on {
            poe_disp.update(
                &stats.ip_address,
                stats.cpu_usage,
                stats.cpu_temp_str,
                stats.ram_usage,
                &stats.disk_usage,
                app_state.shift_offset,
            )
            .map_err(|e| format!("Display update error: {:?}", e))?;
        }

        thread::sleep(config.refresh_interval());
    }
}


fn update_display_state(
    config: &Config,
    start_time: Instant,
    now: Instant,
    screen_timeout_duration: Duration,
    periodic_on_duration: Duration,
    periodic_off_duration: Duration,
    state: &mut AppState,
    poe_disp: &mut PoeDisplay,
) -> Result<(), Box<dyn Error>> {
    let total_elapsed_time = now.duration_since(start_time);
    let time_since_last_toggle = now.duration_since(state.last_periodic_toggle_time);

    if screen_timeout_duration.as_secs() > 0
        && !state.screen_dimmed
        && total_elapsed_time >= screen_timeout_duration
    {
        info!("Screen timeout reached. Dimming display.");
        poe_disp
            .set_brightness(Brightness::DIMMEST)
            .map_err(|e| format!("Failed to dim display: {:?}", e))?;
        state.screen_dimmed = true;
    }

    if config.display.enable_periodic_off {
        if state.is_display_periodically_on && time_since_last_toggle >= periodic_on_duration {
            debug!("Periodic timer: Turning display OFF.");
            poe_disp.display_off()
                .map_err(|e| format!("Failed periodic display OFF: {:?}", e))?;
            state.is_display_periodically_on = false;
            state.last_periodic_toggle_time = now;
        } else if !state.is_display_periodically_on && time_since_last_toggle >= periodic_off_duration {
            debug!("Periodic timer: Turning display ON.");
            poe_disp.display_on()
                .map_err(|e| format!("Failed periodic display ON: {:?}", e))?;
            state.is_display_periodically_on = true;
            state.last_periodic_toggle_time = now;
        }
    }
    Ok(())
}

fn update_pixel_shift(
    now: Instant,
    shift_interval: Duration,
    shift_pattern: &[Point],
    state: &mut AppState,
) {
    if now.duration_since(state.last_shift_time) >= shift_interval {
        state.shift_index = (state.shift_index + 1) % shift_pattern.len();
        state.shift_offset = shift_pattern[state.shift_index];
        state.last_shift_time = now;
        debug!("Shifting display pixels to offset: {:?}", state.shift_offset);
    }
}

fn gather_stats(sys: &mut System) -> SystemStats {
    sys.refresh_cpu_usage();
    sys.refresh_memory();

    let ip_address = get_ip_address();
    let disk_usage = format!("{:.1}", get_disk_usage());
    let cpu_temp = get_cpu_temperature();
    let cpu_temp_str = format!("{:.1}", cpu_temp);
    let cpu_usage = format!("{:.1}", sys.global_cpu_usage());
    let ram_usage = format!("{:.1}", get_ram_usage(sys));

    SystemStats {
        ip_address,
        cpu_usage,
        cpu_temp,
        cpu_temp_str,
        ram_usage,
        disk_usage,
    }
}

fn handle_fan_control(
    fan_controller: &mut FanController,
    cpu_temp: f32,
) -> Result<(), Box<dyn Error>> {
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
    Ok(())
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
