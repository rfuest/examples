//! # Example: Extended characters
//!
//! Demonstrate ability of most built in fonts to render extended characters.

use embedded_graphics::{
    mono_font::{Font12x16, Font24x32, Font6x12, Font6x8, Font8x16, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(1400, 160));

    let test_text  = "¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ";
    let test_text_with_line_break  = "¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏ\nÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ";

    // Show smallest font with black font on white background (default value for fonts)
    Text::new(&format!("Font6x8 {}", test_text), Point::new(15, 15))
        .into_styled(MonoTextStyle::new(Font6x8, BinaryColor::On))
        .draw(&mut display)?;

    // 6x12 font doesn't have support for extended characters
    Text::new(
        "(Font6x12 does not support extended characters)",
        Point::new(15, 30),
    )
    .into_styled(MonoTextStyle::new(Font6x12, BinaryColor::On))
    .draw(&mut display)?;

    // Show 8x16 Font
    Text::new(&format!("Font8x16 {}", test_text), Point::new(15, 45))
        .into_styled(MonoTextStyle::new(Font8x16, BinaryColor::On))
        .draw(&mut display)?;

    // Show 12x16 Font
    Text::new(&format!("Font12x16 {}", test_text), Point::new(15, 65))
        .into_styled(MonoTextStyle::new(Font12x16, BinaryColor::On))
        .draw(&mut display)?;

    // Show 24x32 Font
    Text::new(
        &format!("Font24x32 {}", test_text_with_line_break),
        Point::new(15, 100),
    )
    .into_styled(MonoTextStyle::new(Font24x32, BinaryColor::On))
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}
