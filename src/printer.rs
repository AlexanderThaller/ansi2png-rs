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
    escape::{
        Color,
        ColorType,
    },
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
            // newline
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

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        let params_list: Vec<_> = params.iter().flatten().collect();

        match params_list.as_slice() {
            [value] => match value {
                0 => {
                    let defaults = State::default();

                    self.state.foreground_color = defaults.foreground_color;
                    self.state.font = defaults.font;
                }

                1 => {
                    self.state.font = match self.state.font {
                        FontState::Bold | FontState::Normal => FontState::Bold,
                        FontState::Italic | FontState::ItalicBold => FontState::ItalicBold,
                    }
                }

                3 => {
                    self.state.font = match self.state.font {
                        FontState::Italic | FontState::Normal => FontState::Italic,
                        FontState::Bold | FontState::ItalicBold => FontState::ItalicBold,
                    }
                }

                32 => self.state.foreground_color = ColorType::Normal(Color::Green),
                33 => self.state.foreground_color = ColorType::Normal(Color::Yellow),

                other => {
                    println!(
                        "[csi_dispatch] params={:#?}, intermediates={:?}, ignore={:?}, char={:?}",
                        params, intermediates, ignore, c
                    );

                    dbg!(other);
                }
            },

            [1, 31] => {
                self.state.foreground_color = ColorType::Normal(Color::Red);
                self.state.font = match self.state.font {
                    FontState::Bold | FontState::Normal => FontState::Bold,
                    FontState::Italic | FontState::ItalicBold => FontState::ItalicBold,
                }
            }

            [1, 34] => {
                self.state.foreground_color = ColorType::Normal(Color::Blue);
                self.state.font = match self.state.font {
                    FontState::Bold | FontState::Normal => FontState::Bold,
                    FontState::Italic | FontState::ItalicBold => FontState::ItalicBold,
                }
            }

            [38, 5, value] => match value {
                2 => self.state.foreground_color = ColorType::Normal(Color::Green),

                other => {
                    println!(
                        "[csi_dispatch] params={:#?}, intermediates={:?}, ignore={:?}, char={:?}",
                        params, intermediates, ignore, c
                    );

                    dbg!(other);
                }
            },

            other => {
                println!(
                    "[csi_dispatch] params={:#?}, intermediates={:?}, ignore={:?}, char={:?}",
                    params, intermediates, ignore, c
                );

                dbg!(other);
            }
        }
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        println!(
            "[esc_dispatch] intermediates={:?}, ignore={:?}, byte={:02x}",
            intermediates, ignore, byte
        );
    }
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

        printer.state.text.iter().for_each(|((x, y), entry)| {
            let font = match entry.font {
                FontState::Normal => &printer.settings.font,
                FontState::Bold => &printer.settings.font_bold,
                FontState::Italic => &printer.settings.font_italic,
                FontState::ItalicBold => &printer.settings.font_italic_bold,
            };

            // TODO: Draw background before text

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
