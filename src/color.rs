#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    DarkBlue,
    DarkPurple,
    DarkGreen,
    Brown,
    DarkGrey,
    LightGrey,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Lavender,
    Pink,
    LightPeach,
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => 0,
            Color::DarkBlue => 1,
            Color::DarkPurple => 2,
            Color::DarkGreen => 3,
            Color::Brown => 4,
            Color::DarkGrey => 5,
            Color::LightGrey => 6,
            Color::White => 7,
            Color::Red => 8,
            Color::Orange => 9,
            Color::Yellow => 10,
            Color::Green => 11,
            Color::Blue => 12,
            Color::Lavender => 13,
            Color::Pink => 14,
            Color::LightPeach => 15,
        }
    }
}

impl From<u8> for Color {
    fn from(n: u8) -> Self {
        match n {
            0 => Color::Black,
            1 => Color::DarkBlue,
            2 => Color::DarkPurple,
            3 => Color::DarkGreen,
            4 => Color::Brown,
            5 => Color::DarkGrey,
            6 => Color::LightGrey,
            7 => Color::White,
            8 => Color::Red,
            9 => Color::Orange,
            10 => Color::Yellow,
            11 => Color::Green,
            12 => Color::Blue,
            13 => Color::Lavender,
            14 => Color::Pink,
            15 => Color::LightPeach,
            _ => Color::Black,
        }
    }
}

impl Color {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Black => (0, 0, 0),
            Color::DarkBlue => (29, 43, 83),
            Color::DarkPurple => (126, 37, 83),
            Color::DarkGreen => (0, 135, 81),
            Color::Brown => (171, 82, 54),
            Color::DarkGrey => (95, 87, 79),
            Color::LightGrey => (194, 195, 199),
            Color::White => (255, 241, 232),
            Color::Red => (255, 0, 77),
            Color::Orange => (255, 163, 0),
            Color::Yellow => (255, 236, 39),
            Color::Green => (0, 228, 54),
            Color::Blue => (41, 173, 255),
            Color::Lavender => (131, 118, 156),
            Color::Pink => (255, 119, 168),
            Color::LightPeach => (255, 204, 170),
        }
    }
}
