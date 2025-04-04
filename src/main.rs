use clap::Parser as _;
use image::RgbImage;
use include_flate::flate;
use rusttype::{
    Font,
    Scale,
};
use std::io::Read;
use vte::Parser;

mod color;
mod escape;
mod opt;
mod pallete;
mod printer;

use crate::{
    opt::Opt,
    pallete::Palette,
    printer::Settings,
};

fn main() {
    let opt = Opt::parse();

    let mut input = std::io::BufReader::new(std::fs::File::open(opt.input_path).unwrap());

    flate!(static FONT: [u8] from
        "fonts/iosevka-term-extended.ttf");

    flate!(static FONT_BOLD: [u8] from
        "fonts/iosevka-term-extendedbold.ttf");

    flate!(static FONT_ITALIC: [u8] from
        "fonts/iosevka-term-extendeditalic.ttf");

    flate!(static FONT_ITALIC_BOLD: [u8] from
        "fonts/iosevka-term-extendedbolditalic.ttf");

    let font = Font::try_from_bytes(&FONT).unwrap();
    let font_bold = Font::try_from_bytes(&FONT_BOLD).unwrap();
    let font_italic = Font::try_from_bytes(&FONT_ITALIC).unwrap();
    let font_italic_bold = Font::try_from_bytes(&FONT_ITALIC_BOLD).unwrap();

    let font_height = 50.0;
    let scale = Scale {
        x: font_height,
        y: font_height,
    };

    let pallete = Palette::Custom;
    let png_width = opt.png_width;

    let mut statemachine = Parser::new();
    let mut performer = printer::new(Settings {
        font,
        font_bold,
        font_italic,
        font_italic_bold,
        font_height,
        scale,
        pallete,
        png_width,
    });

    let mut buf = [0; 2048];

    loop {
        match input.read(&mut buf) {
            Ok(0) => break,

            Ok(n) => {
                for byte in &buf[..n] {
                    statemachine.advance(&mut performer, *byte);
                }
            }

            Err(err) => {
                println!("err: {err}");
                break;
            }
        }
    }

    let image: RgbImage = performer.into();
    image.save(opt.output_path).unwrap();
}
