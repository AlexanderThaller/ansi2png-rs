/// From https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters

#[derive(Debug, Clone, Copy)]
pub(super) enum EscapeSequence {
    Reset,

    BlackletterFont,
    Bold,
    Faint,
    Italic,
    RapidBlink,
    SlowBlink,
    Underline,

    NotBold,
    NotUnderlined,
    NormalItensity,
    NotItalicNorBlackletter,
    NotBlinking,

    ReverseVideo,
    Conceal,
    CrossedOut,

    SetForegroundColor,
    DefaultForegroundColor,

    SetBackgroundColor,
    DefaultBackgroundColor,

    PrimaryFont,
    SetAlternativeFont,

    Foreground(ColorType),
    Background(ColorType),

    DisableProportionalSpacing,
    NeitherSuperscriptNorSubscript,

    NotReserved,
    Unimplemented(u8),
}

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

impl From<&u8> for EscapeSequence {
    fn from(value: &u8) -> Self {
        match value {
            0 => Self::Reset,
            1 => Self::Bold,
            2 => Self::Faint,
            3 => Self::Italic,
            4 => Self::Underline,
            5 => Self::SlowBlink,
            6 => Self::RapidBlink,

            7 => Self::ReverseVideo,
            8 => Self::Conceal,
            9 => Self::CrossedOut,

            10 => Self::PrimaryFont,

            11 => Self::SetAlternativeFont,
            12 => Self::SetAlternativeFont,
            13 => Self::SetAlternativeFont,
            14 => Self::SetAlternativeFont,
            15 => Self::SetAlternativeFont,
            16 => Self::SetAlternativeFont,
            17 => Self::SetAlternativeFont,
            18 => Self::SetAlternativeFont,
            19 => Self::SetAlternativeFont,

            20 => Self::BlackletterFont,
            21 => Self::NotBold,
            22 => Self::NormalItensity,
            23 => Self::NotItalicNorBlackletter,
            24 => Self::NotUnderlined,
            25 => Self::NotBlinking,

            27 => Self::NotReserved,

            30 => Self::Foreground(ColorType::Normal(Color::Black)),
            31 => Self::Foreground(ColorType::Normal(Color::Red)),
            32 => Self::Foreground(ColorType::Normal(Color::Green)),
            33 => Self::Foreground(ColorType::Normal(Color::Yellow)),
            34 => Self::Foreground(ColorType::Normal(Color::Blue)),
            35 => Self::Foreground(ColorType::Normal(Color::Magenta)),
            36 => Self::Foreground(ColorType::Normal(Color::Cyan)),
            37 => Self::Foreground(ColorType::Normal(Color::White)),

            38 => Self::SetForegroundColor,
            39 => Self::DefaultForegroundColor,

            40 => Self::Background(ColorType::Normal(Color::Black)),
            41 => Self::Background(ColorType::Normal(Color::Red)),
            42 => Self::Background(ColorType::Normal(Color::Green)),
            43 => Self::Background(ColorType::Normal(Color::Yellow)),
            44 => Self::Background(ColorType::Normal(Color::Blue)),
            45 => Self::Background(ColorType::Normal(Color::Magenta)),
            46 => Self::Background(ColorType::Normal(Color::Cyan)),
            47 => Self::Background(ColorType::Normal(Color::White)),

            48 => Self::SetBackgroundColor,
            49 => Self::DefaultBackgroundColor,
            50 => Self::DisableProportionalSpacing,

            75 => Self::NeitherSuperscriptNorSubscript,

            90 => Self::Background(ColorType::Bright(Color::Black)),
            91 => Self::Background(ColorType::Bright(Color::Red)),
            92 => Self::Background(ColorType::Bright(Color::Green)),
            93 => Self::Background(ColorType::Bright(Color::Yellow)),
            94 => Self::Background(ColorType::Bright(Color::Blue)),
            95 => Self::Background(ColorType::Bright(Color::Magenta)),
            96 => Self::Background(ColorType::Bright(Color::Cyan)),
            97 => Self::Background(ColorType::Bright(Color::White)),

            100 => Self::Background(ColorType::Bright(Color::Black)),
            101 => Self::Background(ColorType::Bright(Color::Red)),
            102 => Self::Background(ColorType::Bright(Color::Green)),
            103 => Self::Background(ColorType::Bright(Color::Yellow)),
            104 => Self::Background(ColorType::Bright(Color::Blue)),
            105 => Self::Background(ColorType::Bright(Color::Magenta)),
            106 => Self::Background(ColorType::Bright(Color::Cyan)),
            107 => Self::Background(ColorType::Bright(Color::White)),

            escape_sequence => Self::Unimplemented(*escape_sequence),
        }
    }
}
