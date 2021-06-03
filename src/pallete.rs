pub trait Palette {
    fn pallete() -> PaletteData;
}

#[derive(Debug)]
pub struct PaletteData {
    pub primary_background: [u8; 3],
    pub primary_foreground: [u8; 3],

    pub foreground_blue: [u8; 3],
    pub foreground_green: [u8; 3],
    pub foreground_red: [u8; 3],
    pub foreground_yellow: [u8; 3],
}

pub struct Custom {}

impl Palette for Custom {
    fn pallete() -> PaletteData {
        PaletteData {
            // primary_background: "0x161616".parse().unwrap(),
            // primary_foreground: "0xf2f2f2".parse().unwrap(),
            primary_background: [22, 22, 22],
            primary_foreground: [242, 242, 242],

            foreground_blue: [21, 101, 193],
            foreground_green: [85, 139, 46],
            foreground_red: [198, 40, 40],
            foreground_yellow: [249, 168, 37],
        }
    }
}
