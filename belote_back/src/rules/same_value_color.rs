use crate::card::Card;
use crate::rule::into_next;
use crate::rule::Rule;

/// ## SameValueColor
///
/// A rule that handles two cards with the same value and color.
#[derive(Default)]
pub struct SameValueColor {
    next: Option<Box<dyn Rule>>,
}

impl SameValueColor {
    /// ## Create a new SameValueColor rule.
    pub fn new(next: Option<impl Rule + 'static>) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Rule for SameValueColor {
    /// Function to check if the rule can handle the cards.
    fn can_handle(&self, cards: (&Card, &Card)) -> bool {
        cards.0.get_value() == cards.1.get_value() && cards.0.get_color() == cards.1.get_color()
    }

    /// Function to handle the cards.
    fn handle(&self, cards: (&Card, &Card)) -> i32 {
        (cards.0.get_value() + cards.1.get_value()) * -2
    }

    /// Function to get the next node in the chain of responsibility.
    fn next(&self) -> &Option<Box<dyn Rule>> {
        &self.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::color::Color;
    use crate::card::value::Value;

    #[test]
    fn same_value_color_can_handle() {
        let rule = SameValueColor::default();
        let card1 = Card::new(Color::Spade, Value::Number(7));
        let card2 = Card::new(Color::Spade, Value::Number(7));
        assert!(rule.can_handle((&card1, &card2)));
    }

    #[test]
    fn same_value_color_can_handle_false() {
        let rule = SameValueColor::default();
        let card1 = Card::new(Color::Spade, Value::Number(1));
        let card2 = Card::new(Color::Heart, Value::Number(2));
        assert!(!rule.can_handle((&card1, &card2)));
    }

    #[test]
    fn same_value_color_handle() {
        let rule = SameValueColor::default();
        let card1 = Card::new(Color::Spade, Value::Ace);
        let card2 = Card::new(Color::Spade, Value::King);
        assert_eq!(rule.handle((&card1, &card2)), -30);
    }
}
