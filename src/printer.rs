use image::{
    Rgb,
    RgbImage,
};
use imageproc::drawing::draw_text_mut;
use rusttype::{
    Font,
    Scale,
};
use std::collections::BTreeMap;
use vte::{
    Params,
    Perform,
};

use crate::{
    color::ColorType,
    escape::EscapeSequence,
    pallete::Palette,
};

pub(super) struct Settings<'a> {
    pub(super) font: Font<'a>,
    pub(super) font_bold: Font<'a>,
    pub(super) font_italic: Font<'a>,
    pub(super) font_italic_bold: Font<'a>,
    pub(super) font_height: f32,
    pub(super) scale: Scale,
    pub(super) pallete: Palette,
}

#[derive(Debug, Default)]
struct SettingsInternal {
    glyph_advance_width: f32,
    new_line_distance: u32,
}

#[derive(Debug)]
struct TextEntry {
    character: char,
    foreground_color: ColorType,
    background_color: ColorType,
    font: FontState,
}

#[derive(Debug, Clone, Copy)]
enum FontState {
    Normal,
    Bold,
    Italic,
    ItalicBold,
}

#[derive(Debug)]
struct State {
    text: BTreeMap<(u32, u32), TextEntry>,
    current_x: u32,
    current_y: u32,
    foreground_color: ColorType,
    background_color: ColorType,
    font: FontState,
    last_execute_byte: Option<u8>,
}

pub(super) struct Printer<'a> {
    settings: Settings<'a>,
    settings_internal: SettingsInternal,
    state: State,
}

pub(super) fn new(settings: Settings) -> Printer {
    let glyph_advance_width = settings
        .font
        .glyph('_')
        .scaled(settings.scale)
        .h_metrics()
        .advance_width;

    let new_line_distance = settings.font_height as u32 - 7;

    let settings_internal = SettingsInternal {
        glyph_advance_width,
        new_line_distance,
    };

    Printer {
        settings,
        settings_internal,
        state: State::default(),
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            text: BTreeMap::new(),
            current_x: 0,
            current_y: 0,
            foreground_color: ColorType::PrimaryForeground,
            background_color: ColorType::PrimaryBackground,
            font: FontState::Normal,
            last_execute_byte: None,
        }
    }
}

impl<'a> Perform for Printer<'a> {
    fn print(&mut self, character: char) {
        self.state.text.insert(
            (self.state.current_x, self.state.current_y),
            TextEntry {
                character,
                foreground_color: self.state.foreground_color,
                background_color: self.state.background_color,
                font: self.state.font,
            },
        );

        self.state.current_x += self.settings_internal.glyph_advance_width as u32;
    }

    fn execute(&mut self, byte: u8) {
        if let Some(last_execute_byte) = self.state.last_execute_byte {
            // Skip printing another newline when `\n\r` was found.
            if byte == 0x0a && last_execute_byte == 0x0d {
                return;
            }
        }

        match byte {
            // newlines
            0x0d | 0x0a => {
                self.state.current_x = 0;
                self.state.current_y += self.settings_internal.new_line_distance;
            }

            _ => println!("[execute] {byte}, {byte:02x}", byte = byte),
        }

        self.state.last_execute_byte = Some(byte)
    }

    fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        println!(
            "[hook] params={:?}, intermediates={:?}, ignore={:?}, char={:?}",
            params, intermediates, ignore, c
        );
    }

    fn put(&mut self, byte: u8) {
        println!("[put] {:02x}", byte);
    }

    fn unhook(&mut self) {
        println!("[unhook]");
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        println!(
            "[osc_dispatch] params={:?} bell_terminated={}",
            params, bell_terminated
        );
    }

    fn csi_dispatch(&mut self, params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {
        let actions = EscapeSequence::parse_params(params);

        for action in actions {
            match action {
                EscapeSequence::Reset => {
                    let defaults = State::default();

                    self.state.foreground_color = defaults.foreground_color;
                    self.state.background_color = defaults.background_color;
                    self.state.font = defaults.font;
                }

                EscapeSequence::Bold => self.state.font += FontState::Bold,
                EscapeSequence::Italic => self.state.font += FontState::Italic,
                EscapeSequence::NotBold => self.state.font -= FontState::Bold,
                EscapeSequence::NotItalicNorBlackletter => self.state.font -= FontState::Italic,

                EscapeSequence::ForegroundColor(color_type) => {
                    self.state.foreground_color = color_type
                }
                EscapeSequence::BackgroundColor(color_type) => {
                    self.state.background_color = color_type
                }

                EscapeSequence::DefaultForegroundColor => {
                    self.state.foreground_color = ColorType::PrimaryForeground
                }

                EscapeSequence::DefaultBackgroundColor => {
                    self.state.background_color = ColorType::PrimaryBackground
                }

                EscapeSequence::BlackletterFont
                | EscapeSequence::Faint
                | EscapeSequence::SlowBlink
                | EscapeSequence::Underline
                | EscapeSequence::NotUnderline
                | EscapeSequence::NotBlinking
                | EscapeSequence::ReverseVideo
                | EscapeSequence::Conceal
                | EscapeSequence::CrossedOut
                | EscapeSequence::PrimaryFont
                | EscapeSequence::SetAlternativeFont
                | EscapeSequence::DisableProportionalSpacing
                | EscapeSequence::NeitherSuperscriptNorSubscript
                | EscapeSequence::NotReserved
                | EscapeSequence::NormalItensity
                | EscapeSequence::RapidBlink => {
                    // eprintln!("not implemented for action: {:?}", action)
                }
                EscapeSequence::Unimplemented(value) => {
                    eprintln!("not implemented for value: {:?}", value)
                }
            }
        }
    }

    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {}
}

impl<'a> From<Printer<'a>> for RgbImage {
    fn from(printer: Printer) -> Self {
        let width = printer
            .state
            .text
            .keys()
            .map(|(x, _)| x)
            .max()
            .unwrap_or(&0)
            + printer.settings_internal.glyph_advance_width as u32;

        let height = printer
            .state
            .text
            .keys()
            .map(|(_, y)| y)
            .max()
            .unwrap_or(&0)
            + printer.settings_internal.new_line_distance * 2
            - 30;

        let mut image = RgbImage::new(width, height);

        // Set primary background
        for (_x, _y, pixel) in image.enumerate_pixels_mut() {
            *pixel = image::Rgb(
                printer
                    .settings
                    .pallete
                    .get_color(ColorType::PrimaryBackground),
            );
        }

        // Render background before foreground
        printer.state.text.iter().for_each(|((x, y), entry)| {
            let background_end_x = x + printer.settings_internal.glyph_advance_width as u32;
            let background_end_y = y + printer.settings.font_height as u32;

            for x in *x..background_end_x {
                for y in *y..background_end_y {
                    let pixel =
                        image::Rgb(printer.settings.pallete.get_color(entry.background_color));

                    image.put_pixel(x, y, pixel);
                }
            }
        });

        printer.state.text.iter().for_each(|((x, y), entry)| {
            let font = match entry.font {
                FontState::Normal => &printer.settings.font,
                FontState::Bold => &printer.settings.font_bold,
                FontState::Italic => &printer.settings.font_italic,
                FontState::ItalicBold => &printer.settings.font_italic_bold,
            };

            draw_text_mut(
                &mut image,
                Rgb(printer.settings.pallete.get_color(entry.foreground_color)),
                *x,
                *y,
                printer.settings.scale,
                font,
                &entry.character.to_string(),
            )
        });

        image
    }
}

impl std::ops::AddAssign for FontState {
    fn add_assign(&mut self, other: Self) {
        let new_self = match (&self, other) {
            (Self::Normal, Self::Normal) => Self::Normal,

            (Self::Bold, Self::Bold) | (Self::Bold, Self::Normal) | (Self::Normal, Self::Bold) => {
                Self::Bold
            }

            (Self::Italic, Self::Italic)
            | (Self::Italic, Self::Normal)
            | (Self::Normal, Self::Italic) => Self::Italic,

            (Self::Bold, Self::Italic)
            | (Self::Bold, Self::ItalicBold)
            | (Self::ItalicBold, Self::Bold)
            | (Self::ItalicBold, Self::Italic)
            | (Self::ItalicBold, Self::ItalicBold)
            | (Self::ItalicBold, Self::Normal)
            | (Self::Italic, Self::Bold)
            | (Self::Italic, Self::ItalicBold)
            | (Self::Normal, Self::ItalicBold) => Self::ItalicBold,
        };

        *self = new_self
    }
}

impl std::ops::SubAssign for FontState {
    fn sub_assign(&mut self, other: Self) {
        let new_self = match (&self, other) {
            (Self::Italic, Self::Italic)
            | (Self::ItalicBold, Self::ItalicBold)
            | (Self::Bold, Self::Bold)
            | (Self::Normal, Self::Normal)
            | (Self::Normal, Self::Bold)
            | (Self::Normal, Self::Italic)
            | (Self::Bold, Self::ItalicBold)
            | (Self::Italic, Self::ItalicBold)
            | (Self::Normal, Self::ItalicBold) => Self::Normal,

            (Self::Bold, Self::Normal)
            | (Self::Bold, Self::Italic)
            | (Self::ItalicBold, Self::Italic) => Self::Bold,

            (Self::Italic, Self::Normal)
            | (Self::Italic, Self::Bold)
            | (Self::ItalicBold, Self::Bold) => Self::Italic,

            (Self::ItalicBold, Self::Normal) => Self::ItalicBold,
        };

        *self = new_self
    }
}
