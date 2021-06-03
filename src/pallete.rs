#[derive(Debug)]
pub struct Palette {
    pub primary_background: [u8; 3],
    pub primary_foreground: [u8; 3],

    pub foreground_blue: [u8; 3],
    pub foreground_green: [u8; 3],
    pub foreground_red: [u8; 3],
}

pub struct Custom {}

impl From<Custom> for Palette {
    fn from(_: Custom) -> Self {
        Palette {
            // primary_background: "0x161616".parse().unwrap(),
            // primary_foreground: "0xf2f2f2".parse().unwrap(),
            primary_background: [22, 22, 22],
            primary_foreground: [242, 242, 242],

            foreground_blue: [21, 101, 193],
            foreground_green: [85, 139, 46],
            foreground_red: [198, 40, 40],
        }
    }
}
