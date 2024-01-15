use crate::{card::Card, error::GameError};

/// ## Rule
///
/// A trait that represents a chain of responsibility for the game rules.
///
/// The rule is executed by calling the `execute` method.
/// If the rule can handle the cards, it will return the score.
/// If it cannot handle the cards, it will call the `next` method to get the next rule and execute it.
/// If there is no next rule, it will return an error.
pub trait Rule {
    fn execute(&self, cards: (&Card, &Card)) -> Result<i32, GameError> {
        if self.can_handle(cards) {
            Ok(self.handle(cards))
        } else if let Some(next) = &mut self.next() {
            next.execute(cards)
        } else {
            Err(GameError::CannotHandleCards)
        }
    }

    fn can_handle(&self, cards: (&Card, &Card)) -> bool;
    fn handle(&self, cards: (&Card, &Card)) -> i32;
    fn next(&self) -> &Option<Box<dyn Rule>>;
}

/// ## A function to create the next node in the chain of responsibility.
pub fn into_next(rule: Option<impl Rule + Sized + 'static>) -> Option<Box<dyn Rule>> {
    match rule {
        Some(r) => Some(Box::new(r)),
        None => None,
    }
}
