use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettings, OutputSettingsBuilder, SimulatorDisplay,
};
use rustberry_poe_monitor::render;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ip_address = "192.168.1.100".to_string();
    let mut cpu_usage = "12.3".to_string();
    let mut temp = "45.6".to_string();
    let mut ram_usage = "34.5".to_string();
    let mut disk_usage = "56.7".to_string();
    let mut offset = Point::zero();
    let mut png_path = None;

    for arg in env::args().skip(1) {
        let (key, value) = arg
            .split_once('=')
            .ok_or_else(|| format!("expected key=value, got: {arg}"))?;
        match key {
            "ip" => ip_address = value.to_string(),
            "cpu" => cpu_usage = value.to_string(),
            "temp" => temp = value.to_string(),
            "ram" => ram_usage = value.to_string(),
            "disk" => disk_usage = value.to_string(),
            "shift" => offset = Point::new(value.parse()?, 0),
            "png" => png_path = Some(value.to_string()),
            _ => return Err(format!("unknown option: {key}").into()),
        }
    }

    let mut display = SimulatorDisplay::<BinaryColor>::new(render::DISPLAY_SIZE);
    render::draw(
        &mut display,
        &ip_address,
        &cpu_usage,
        &temp,
        &ram_usage,
        &disk_usage,
        offset,
    )?;

    let settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .scale(4)
        .pixel_spacing(1)
        .build();

    match png_path {
        Some(path) => display.to_rgb_output_image(&settings).save_png(path)?,
        None => show_window(&display, &settings)?,
    }
    Ok(())
}

#[cfg(feature = "window")]
fn show_window(
    display: &SimulatorDisplay<BinaryColor>,
    settings: &OutputSettings,
) -> Result<(), Box<dyn Error>> {
    embedded_graphics_simulator::Window::new("RustBerry PoE Monitor", settings)
        .show_static(display);
    Ok(())
}

#[cfg(not(feature = "window"))]
fn show_window(
    _display: &SimulatorDisplay<BinaryColor>,
    _settings: &OutputSettings,
) -> Result<(), Box<dyn Error>> {
    Err("built without the window feature, use png=<path> instead".into())
}
