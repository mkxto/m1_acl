use color::Color;
use core::fmt;
use value::Value;

pub mod color;
pub mod value;

/// ## Card
///
/// A structure that represents a card in the game.
#[derive(Debug, Clone)]
pub struct Card {
    color: Color,
    value: Value,
}

impl Card {
    /// ## Create a new Card given a color and a value.
    ///
    /// Returns a new Card with the given color and value.
    pub fn new(color: Color, value: Value) -> Card {
        Card { color, value }
    }

    /// ## Get the value of the Card.
    ///
    /// Returns the value of the Card.
    pub fn get_value(&self) -> Value {
        self.value
    }

    /// ## Get the color of the Card.
    ///
    /// Returns the color of the Card.
    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_id(&self) -> String {
        format!("{}{}", self.color, self.value)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.value == other.value && self.color == other.color
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.color, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_new() {
        let card = Card::new(Color::Spade, Value::Number(1));
        assert_eq!(card.get_value(), Value::Number(1));
        assert_eq!(card.get_color(), Color::Spade);
    }

    #[test]
    fn card_eq() {
        let card1 = Card::new(Color::Spade, Value::Number(1));
        let card2 = Card::new(Color::Spade, Value::Number(1));
        assert_eq!(card1, card2);
    }

    #[test]
    fn card_neq() {
        let card1 = Card::new(Color::Spade, Value::Number(1));
        let card2 = Card::new(Color::Heart, Value::Number(1));
        assert_ne!(card1, card2);
    }

    #[test]
    fn card_get_id() {
        let card = Card::new(Color::Spade, Value::Number(1));
        assert_eq!(card.get_id(), "S1");
    }

    #[test]
    fn card_display() {
        let card = Card::new(Color::Spade, Value::Number(1));
        assert_eq!(card.to_string(), "Spade 1");
    }
}
