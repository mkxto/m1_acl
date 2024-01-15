use core::fmt;

/// ## Color
///
/// An enum that represents the color of a card.
/// The color of a card is in fact the suit of a card.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = match self {
            Color::Spade => "S",
            Color::Heart => "H",
            Color::Diamond => "D",
            Color::Club => "C",
        };
        write!(f, "{}", color)
    }
}
