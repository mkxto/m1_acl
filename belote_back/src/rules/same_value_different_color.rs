use crate::card::Card;
use crate::rule::into_next;
use crate::rule::Rule;

/// ## SameValueDifferentColor
///
/// A rule that handles two cards with the same value and different colors.
#[derive(Default)]
pub struct SameValueDifferentColor {
    next: Option<Box<dyn Rule>>,
}

impl SameValueDifferentColor {
    /// ## Create a new SameValueDifferentColor rule.
    pub fn new(next: Option<impl Rule + 'static>) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Rule for SameValueDifferentColor {
    /// Function to check if the rule can handle the cards.
    fn can_handle(&self, cards: (&Card, &Card)) -> bool {
        cards.0.get_value() == cards.1.get_value() && cards.0.get_color() != cards.1.get_color()
    }

    /// Function to handle the cards.
    fn handle(&self, cards: (&Card, &Card)) -> i32 {
        -(cards.0.get_value() + cards.1.get_value())
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
    fn same_value_different_color_can_handle() {
        let rule = SameValueDifferentColor::default();
        let card1 = Card::new(Color::Spade, Value::Number(1));
        let card2 = Card::new(Color::Heart, Value::Number(1));
        assert!(rule.can_handle((&card1, &card2)));
    }

    #[test]
    fn same_value_different_color_can_handle_false() {
        let rule = SameValueDifferentColor::default();
        let card1 = Card::new(Color::Spade, Value::Number(1));
        let card2 = Card::new(Color::Spade, Value::Number(2));
        assert!(!rule.can_handle((&card1, &card2)));
    }

    #[test]
    fn same_value_different_color_handle() {
        let rule = SameValueDifferentColor::default();
        let card1 = Card::new(Color::Spade, Value::Number(9));
        let card2 = Card::new(Color::Heart, Value::Jack);
        assert_eq!(rule.handle((&card1, &card2)), -2);
    }
}
