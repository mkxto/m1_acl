use crate::card::Card;
use crate::rule::into_next;
use crate::rule::Rule;

/// ## DifferentValueColor
///
/// A rule that handles two cards with different values and different colors.
#[derive(Default)]
pub struct DifferentValueColor {
    next: Option<Box<dyn Rule>>,
}

impl DifferentValueColor {
    /// ## Create a new DifferentValueColor rule.
    pub fn new(next: Option<impl Rule + 'static>) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Rule for DifferentValueColor {
    /// Function to check if the rule can handle the cards.
    fn can_handle(&self, cards: (&Card, &Card)) -> bool {
        cards.0.get_value() != cards.1.get_value() && cards.0.get_color() != cards.1.get_color()
    }

    /// Function to handle the cards.
    fn handle(&self, cards: (&Card, &Card)) -> i32 {
        cards.0.get_value() + cards.1.get_value()
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
    fn different_value_color_can_handle() {
        let rule = DifferentValueColor::default();
        let card1 = Card::new(Color::Spade, Value::Number(7));
        let card2 = Card::new(Color::Heart, Value::Number(8));
        assert!(rule.can_handle((&card1, &card2)));
    }

    #[test]
    fn different_value_color_cant_handle() {
        let rule = DifferentValueColor::default();
        let card1 = Card::new(Color::Spade, Value::Number(7));
        let card2 = Card::new(Color::Spade, Value::Number(7));
        assert!(!rule.can_handle((&card1, &card2)));
    }

    #[test]
    fn different_value_color_handle() {
        let rule = DifferentValueColor::default();
        let card1 = Card::new(Color::Spade, Value::Number(7));
        let card2 = Card::new(Color::Heart, Value::Number(8));
        assert!(rule.can_handle((&card1, &card2)));
        assert_eq!(rule.handle((&card1, &card2)), 0);
    }

    #[test]
    fn sum() {
        let rule = DifferentValueColor::default();
        let card1 = Card::new(Color::Spade, Value::Ace);
        let card2 = Card::new(Color::Heart, Value::Jack);
        assert_eq!(rule.handle((&card1, &card2)), 13);
    }

    #[test]
    fn different_value_color_next() {
        let rule = DifferentValueColor::default();
        assert!(rule.next().is_none());
    }
}
