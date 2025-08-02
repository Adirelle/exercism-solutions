#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    match color {
        ResistorColor::Black => 0,
        ResistorColor::Blue => 6,
        ResistorColor::Brown => 1,
        ResistorColor::Green => 5,
        ResistorColor::Grey => 8,
        ResistorColor::Orange => 3,
        ResistorColor::Red => 2,
        ResistorColor::Violet => 7,
        ResistorColor::White => 9,
        ResistorColor::Yellow => 4,
    }
}

pub fn value_to_color_string(value: u32) -> String {
    String::from(match value {
        0 => "Black",
        1 => "Brown",
        2 => "Red",
        3 => "Orange",
        4 => "Yellow",
        5 => "Green",
        6 => "Blue",
        7 => "Violet",
        8 => "Grey",
        9 => "White",
        _ => "value out of range",
    })
}

pub fn colors() -> Vec<ResistorColor> {
    vec![
        ResistorColor::Black,
        ResistorColor::Brown,
        ResistorColor::Red,
        ResistorColor::Orange,
        ResistorColor::Yellow,
        ResistorColor::Green,
        ResistorColor::Blue,
        ResistorColor::Violet,
        ResistorColor::Grey,
        ResistorColor::White,
    ]
}
