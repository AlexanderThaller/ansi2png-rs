pub trait Palette {
    fn pallete() -> PaletteData;
}

#[derive(Debug)]
pub struct PaletteData {
    pub primary_foreground: [u8; 3],
    pub primary_background: [u8; 3],

    pub black: [u8; 3],
    pub red: [u8; 3],
    pub green: [u8; 3],
    pub yellow: [u8; 3],
    pub blue: [u8; 3],
    pub magenta: [u8; 3],
    pub cyan: [u8; 3],
    pub white: [u8; 3],

    pub bright_black: [u8; 3],
    pub bright_red: [u8; 3],
    pub bright_green: [u8; 3],
    pub bright_yellow: [u8; 3],
    pub bright_blue: [u8; 3],
    pub bright_magenta: [u8; 3],
    pub bright_cyan: [u8; 3],
    pub bright_white: [u8; 3],
}

pub struct Custom {}

impl Palette for Custom {
    fn pallete() -> PaletteData {
        PaletteData {
            // primary_background: "0x161616".parse().unwrap(),
            // primary_foreground: "0xf2f2f2".parse().unwrap(),
            primary_foreground: [242, 242, 242],
            primary_background: [22, 22, 22],

            black: [44, 44, 44],
            red: [198, 40, 40],
            green: [85, 139, 46],
            yellow: [249, 168, 37],
            blue: [21, 101, 193],
            magenta: [168, 37, 191],
            cyan: [0, 131, 143],
            white: [255, 255, 255],

            bright_black: [44, 44, 44],
            bright_red: [198, 40, 40],
            bright_green: [85, 139, 46],
            bright_yellow: [249, 168, 37],
            bright_blue: [21, 101, 193],
            bright_magenta: [168, 37, 191],
            bright_cyan: [0, 131, 143],
            bright_white: [255, 255, 255],
        }
    }
}
