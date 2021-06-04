use ansi_parser::{
    AnsiParser,
    AnsiSequence,
    Output,
};
use image::{
    Rgb,
    RgbImage,
};
use imageproc::drawing::draw_text_mut;
use rusttype::{
    Font,
    Scale,
};
use structopt::StructOpt;

mod escape;
mod opt;
mod pallete;

use crate::{
    escape::{
        Color,
        ColorType,
        EscapeSequence,
    },
    opt::Opt,
    pallete::Palette,
};

fn main() {
    let opt = Opt::from_args();

    let input_str = std::fs::read_to_string(opt.input_path).unwrap();
    let parsed: Vec<Output> = input_str.ansi_parse().collect();

    let pallete = pallete::Custom::pallete();

    let font = Vec::from(
        include_bytes!("../resources/dejavu-fonts-ttf-2.37/ttf/DejaVuSansMono.ttf") as &[u8],
    );
    let font_bold = Vec::from(include_bytes!(
        "../resources/dejavu-fonts-ttf-2.37/ttf/DejaVuSansMono-Bold.ttf"
    ) as &[u8]);
    let font_italic = Vec::from(include_bytes!(
        "../resources/dejavu-fonts-ttf-2.37/ttf/DejaVuSansMono-Oblique.ttf"
    ) as &[u8]);

    let font = Font::try_from_vec(font).unwrap();
    let font_bold = Font::try_from_vec(font_bold).unwrap();
    let font_italic = Font::try_from_vec(font_italic).unwrap();

    let height = 50.0;
    let scale = Scale {
        x: height,
        y: height,
    };
    let new_line_distance = height as u32 - 7;
    let glyph_advance_width = font.glyph('_').scaled(scale).h_metrics().advance_width;

    let lines_count = parsed
        .iter()
        .filter_map(|block| {
            if let Output::TextBlock(text) = block {
                Some(text.to_string())
            } else {
                None
            }
        })
        .collect::<String>()
        .lines()
        .count();

    let max_characters = parsed
        .iter()
        .filter_map(|block| {
            if let Output::TextBlock(text) = block {
                Some(text.to_string())
            } else {
                None
            }
        })
        .collect::<String>()
        .lines()
        .map(|line| line.len())
        .max()
        .unwrap_or_default();

    let width: u32 = (max_characters as f32 * glyph_advance_width).ceil() as u32;
    let height = (lines_count) as u32 * new_line_distance - 30;

    let mut image = RgbImage::new(width, height);

    // Set background
    for (_x, _y, pixel) in image.enumerate_pixels_mut() {
        *pixel = image::Rgb(pallete.primary_background);
    }

    let mut draw_x = 0;
    let mut draw_y = 0;
    let mut foreground_color = Rgb(pallete.primary_foreground);
    let mut text_bold = false;
    let mut text_italic = false;
    let mut text_faint = false;
    let mut text_underline = false;

    parsed.iter().for_each(|block| match block {
        Output::TextBlock(text) => text.chars().filter(|c| *c != '\r').for_each(|c| {
            let draw_font = if text_bold { &font_bold } else { &font };
            let draw_font = if text_italic || text_faint || text_underline {
                &font_italic
            } else {
                draw_font
            };

            if c == '\n' {
                draw_x = 0;
                draw_y += new_line_distance
            } else {
                draw_text_mut(
                    &mut image,
                    foreground_color,
                    draw_x,
                    draw_y,
                    scale,
                    draw_font,
                    &c.to_string(),
                );

                let advance_width = font.glyph(c).scaled(scale).h_metrics().advance_width;
                draw_x += advance_width as u32;
            }
        }),

        Output::Escape(escape) => {
            if let AnsiSequence::SetGraphicsMode(values) = escape {
                values
                    .into_iter()
                    .map(|value| {
                        dbg!(value);
                        value.into()
                    })
                    .for_each(|sequence| {
                        dbg!(&sequence);

                        match sequence {
                            EscapeSequence::Reset => {
                                text_bold = false;
                                foreground_color = Rgb(pallete.primary_foreground)
                            }
                            EscapeSequence::Bold => text_bold = true,
                            EscapeSequence::Italic => text_italic = true,
                            EscapeSequence::Faint => text_faint = true,
                            EscapeSequence::Underline => text_underline = true,

                            EscapeSequence::Foreground(color_type) => match color_type {
                                ColorType::Normal(color) => match color {
                                    Color::Black => foreground_color = Rgb(pallete.black),
                                    Color::Red => foreground_color = Rgb(pallete.red),
                                    Color::Green => foreground_color = Rgb(pallete.green),
                                    Color::Yellow => foreground_color = Rgb(pallete.yellow),
                                    Color::Blue => foreground_color = Rgb(pallete.blue),
                                    Color::Magenta => foreground_color = Rgb(pallete.magenta),
                                    Color::Cyan => foreground_color = Rgb(pallete.cyan),
                                    Color::White => foreground_color = Rgb(pallete.white),
                                },

                                ColorType::Bright(color) => match color {
                                    Color::Black => foreground_color = Rgb(pallete.bright_black),
                                    Color::Red => foreground_color = Rgb(pallete.bright_red),
                                    Color::Green => foreground_color = Rgb(pallete.bright_green),
                                    Color::Yellow => foreground_color = Rgb(pallete.bright_yellow),
                                    Color::Blue => foreground_color = Rgb(pallete.bright_blue),
                                    Color::Magenta => {
                                        foreground_color = Rgb(pallete.bright_magenta)
                                    }
                                    Color::Cyan => foreground_color = Rgb(pallete.bright_cyan),
                                    Color::White => foreground_color = Rgb(pallete.bright_white),
                                },
                            },

                            EscapeSequence::Background(color_type) => match color_type {
                                ColorType::Normal(color) => match color {
                                    Color::Black => foreground_color = Rgb(pallete.black),
                                    Color::Red => foreground_color = Rgb(pallete.red),
                                    Color::Green => foreground_color = Rgb(pallete.green),
                                    Color::Yellow => foreground_color = Rgb(pallete.yellow),
                                    Color::Blue => foreground_color = Rgb(pallete.blue),
                                    Color::Magenta => foreground_color = Rgb(pallete.magenta),
                                    Color::Cyan => foreground_color = Rgb(pallete.cyan),
                                    Color::White => foreground_color = Rgb(pallete.white),
                                },

                                ColorType::Bright(color) => match color {
                                    Color::Black => foreground_color = Rgb(pallete.bright_black),
                                    Color::Red => foreground_color = Rgb(pallete.bright_red),
                                    Color::Green => foreground_color = Rgb(pallete.bright_green),
                                    Color::Yellow => foreground_color = Rgb(pallete.bright_yellow),
                                    Color::Blue => foreground_color = Rgb(pallete.bright_blue),
                                    Color::Magenta => {
                                        foreground_color = Rgb(pallete.bright_magenta)
                                    }
                                    Color::Cyan => foreground_color = Rgb(pallete.bright_cyan),
                                    Color::White => foreground_color = Rgb(pallete.bright_white),
                                },
                            },

                            EscapeSequence::Unimplemented(v) => {
                                eprintln!("unimplemented code {}", v)
                            }

                            _ => {}
                        }
                    });
            }
        }
    });

    let _ = image.save(opt.output_path).unwrap();
}
