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
use std::path::Path;

mod pallete;

fn main() {
    let parsed: Vec<Output> = include_str!("../resources/out.ansi").ansi_parse().collect();

    let path = Path::new("output.png");

    let pallete: pallete::Palette = pallete::Custom {}.into();

    let font = Vec::from(
        include_bytes!("../resources/dejavu-fonts-ttf-2.37/ttf/DejaVuSansMono.ttf") as &[u8],
    );
    let font_bold = Vec::from(include_bytes!(
        "../resources/dejavu-fonts-ttf-2.37/ttf/DejaVuSansMono-Bold.ttf"
    ) as &[u8]);

    let font = Font::try_from_vec(font).unwrap();
    let font_bold = Font::try_from_vec(font_bold).unwrap();

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

    parsed.iter().for_each(|block| match block {
        Output::TextBlock(text) => text.chars().for_each(|c| {
            let draw_font = if text_bold { &font_bold } else { &font };

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
                for value in values {
                    match value {
                        0 => {
                            text_bold = false;
                            foreground_color = Rgb(pallete.primary_foreground)
                        }
                        1 => text_bold = true,
                        31 => foreground_color = Rgb(pallete.foreground_red),
                        34 => foreground_color = Rgb(pallete.foreground_blue),
                        _ => {
                            dbg!("not implemented for {}", value);
                        }
                    }
                }
            }
        }
    });

    let _ = image.save(path).unwrap();
}
