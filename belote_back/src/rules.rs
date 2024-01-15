pub mod different_value_color;
pub mod same_value_color;
pub mod same_value_different_color;

use crate::rule::Rule;
use different_value_color::DifferentValueColor;
use same_value_color::SameValueColor;
use same_value_different_color::SameValueDifferentColor;

/// ## A function to create the rules of the game.
pub fn rules() -> impl Rule {
    let same_value_color: SameValueColor = SameValueColor::new(None::<SameValueColor>);
    let same_value_different_color: SameValueDifferentColor =
        SameValueDifferentColor::new(Some(same_value_color));
    DifferentValueColor::new(Some(same_value_different_color))
}

/// ## A function to create the rules of the game with a static lifetime.
pub fn rules_static() -> impl Rule + 'static {
    let same_value_color: SameValueColor = SameValueColor::new(None::<SameValueColor>);
    let same_value_different_color: SameValueDifferentColor =
        SameValueDifferentColor::new(Some(same_value_color));
    DifferentValueColor::new(Some(same_value_different_color))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::color::Color;
    use crate::card::value::Value;
    use crate::card::Card;

    #[test]
    fn different_value_color() {
        let rule = rules();
        let card1 = Card::new(Color::Spade, Value::King);
        let card2 = Card::new(Color::Heart, Value::Number(8));
        assert_eq!(rule.execute((&card1, &card2)).unwrap(), 4);
    }

    #[test]
    fn same_value_color() {
        let rule = rules();
        let card1 = Card::new(Color::Spade, Value::Queen);
        let card2 = Card::new(Color::Spade, Value::Queen);
        assert_eq!(rule.execute((&card1, &card2)).unwrap(), -12);
    }

    #[test]
    fn same_value_different_color() {
        let rule = rules();
        let card1 = Card::new(Color::Spade, Value::Number(10));
        let card2 = Card::new(Color::Heart, Value::Number(10));
        assert_eq!(rule.execute((&card1, &card2)).unwrap(), -20);
    }
}
