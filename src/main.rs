use image::RgbImage;
use rusttype::{
    Font,
    Scale,
};
use std::io::Read;
use structopt::StructOpt;
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
    let opt = Opt::from_args();

    let mut input = std::io::BufReader::new(std::fs::File::open(opt.input_path).unwrap());

    let font = Vec::from(include_bytes!(
        "../resources/ttf-iosevka-term-7.2.6/iosevka-term-extended.ttf"
    ) as &[u8]);

    let font_bold = Vec::from(include_bytes!(
        "../resources/ttf-iosevka-term-7.2.6/iosevka-term-extendedbold.ttf"
    ) as &[u8]);

    let font_italic = Vec::from(include_bytes!(
        "../resources/ttf-iosevka-term-7.2.6/iosevka-term-extendeditalic.ttf"
    ) as &[u8]);

    let font_italic_bold = Vec::from(include_bytes!(
        "../resources/ttf-iosevka-term-7.2.6/iosevka-term-extendedbolditalic.ttf"
    ) as &[u8]);

    let font = Font::try_from_vec(font).unwrap();
    let font_bold = Font::try_from_vec(font_bold).unwrap();
    let font_italic = Font::try_from_vec(font_italic).unwrap();
    let font_italic_bold = Font::try_from_vec(font_italic_bold).unwrap();

    let font_height = 50.0;
    let scale = Scale {
        x: font_height,
        y: font_height,
    };

    let pallete = Palette::Custom;

    let mut statemachine = Parser::new();
    let mut performer = printer::new(Settings {
        font,
        font_bold,
        font_italic,
        font_italic_bold,
        font_height,
        scale,
        pallete,
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
                println!("err: {}", err);
                break;
            }
        }
    }

    let image: RgbImage = performer.into();
    image.save(opt.output_path).unwrap();
}
