#[derive(Debug, Clone, Copy)]
pub(super) enum ColorType {
    PrimaryForeground,
    PrimaryBackground,
    Normal(Color),
    Bright(Color),
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}
