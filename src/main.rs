use ansi_parser::{
    AnsiParser,
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

fn main() {
    let parsed: Vec<Output> = include_str!("../resources/out.ansi").ansi_parse().collect();

    let path = Path::new("output.png");

    let font = Vec::from(
        include_bytes!("../resources/dejavu-fonts-ttf-2.37/ttf/DejaVuSansMono.ttf") as &[u8],
    );
    let font = Font::try_from_vec(font).unwrap();

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

    let draw_x = 0;
    let mut draw_y = 0;

    parsed
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
        .for_each(|line| {
            draw_text_mut(
                &mut image,
                Rgb([255u8, 255u8, 255u8]),
                draw_x,
                draw_y,
                scale,
                &font,
                &line,
            );

            draw_y += new_line_distance;
        });

    let _ = image.save(path).unwrap();
}
