use embedded_graphics::{
    image::ImageRaw,
    mono_font::{
        ascii, mapping::StrGlyphMapping, DecorationDimensions, MonoFont, MonoTextStyle,
        MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use profont::PROFONT_12_POINT;

pub const DISPLAY_SIZE: Size = Size::new(128, 32);

const DISPLAY_WIDTH: i32 = DISPLAY_SIZE.width as i32;
const IP_ROW_Y: i32 = 7;
const STATS_ROW1_Y: i32 = 19;
const STATS_ROW2_Y: i32 = 30;
const LEFT_COL_RIGHT: i32 = 34;
const RIGHT_COL_RIGHT: i32 = 99;
const X_MARGIN: Point = Point::new(2, 0);
const VALUE_CHAR_WIDTH: i32 = PCSENIOR8.character_size.width as i32;

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

const GLYPH_MAPPING: StrGlyphMapping = StrGlyphMapping::new(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~°", '?' as usize - ' ' as usize);

const PCSENIOR8: MonoFont = MonoFont {
    image: ImageRaw::new(include_bytes!("../data/pcsenior.raw"), 128),
    character_size: Size::new(8, 10),
    character_spacing: 0,
    baseline: 7,
    underline: DecorationDimensions::new(9, 1),
    strikethrough: DecorationDimensions::new(10 / 2, 1),
    glyph_mapping: &GLYPH_MAPPING,
};

const PCSENIOR8_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&PCSENIOR8)
    .text_color(BinaryColor::On)
    .build();

pub fn draw<D>(
    target: &mut D,
    ip_address: &str,
    cpu_usage: &str,
    temp: &str,
    ram_usage: &str,
    disk_usage: &str,
    offset: Point,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    target.clear(BinaryColor::Off)?;

    let ip_width = ip_address.len() as i32 * VALUE_CHAR_WIDTH;
    let ip_x_position = (DISPLAY_WIDTH - ip_width) / 2;
    let ip_pos = Point::new(ip_x_position, IP_ROW_Y) + offset;
    Text::new(ip_address, ip_pos, PCSENIOR8_STYLE).draw(target)?;

    let cpu_width = cpu_usage.len() as i32 * VALUE_CHAR_WIDTH;
    let cpu_pos = Point::new(LEFT_COL_RIGHT - cpu_width, STATS_ROW1_Y) + offset;
    let next = Text::new(cpu_usage, cpu_pos, PCSENIOR8_STYLE).draw(target)?;
    let next = Text::new("%", next, FONT_6X12).draw(target)?;
    Text::new("CPU", next + X_MARGIN, FONT_5X8).draw(target)?;

    let ram_width = ram_usage.len() as i32 * VALUE_CHAR_WIDTH;
    let ram_pos = Point::new(LEFT_COL_RIGHT - ram_width, STATS_ROW2_Y) + offset;
    let next = Text::new(ram_usage, ram_pos, PCSENIOR8_STYLE).draw(target)?;
    let next = Text::new("%", next, FONT_6X12).draw(target)?;
    Text::new("RAM", next + X_MARGIN, FONT_5X8).draw(target)?;

    let temp_width = temp.len() as i32 * VALUE_CHAR_WIDTH;
    let temp_pos = Point::new(RIGHT_COL_RIGHT - temp_width, STATS_ROW1_Y) + offset;
    let next = Text::new(temp, temp_pos, PCSENIOR8_STYLE).draw(target)?;
    let next = Text::new("°", next + Point::new(0, 3), PROFONT12).draw(target)?;
    Text::new("C", next - Point::new(0, 2), PCSENIOR8_STYLE).draw(target)?;

    let disk_width = disk_usage.len() as i32 * VALUE_CHAR_WIDTH;
    let disk_pos = Point::new(RIGHT_COL_RIGHT - disk_width, STATS_ROW2_Y) + offset;
    let next = Text::new(disk_usage, disk_pos, PCSENIOR8_STYLE).draw(target)?;
    let next = Text::new("%", next, FONT_6X12).draw(target)?;
    Text::new("DISK", next + X_MARGIN, FONT_5X8).draw(target)?;

    Ok(())
}
