use crate::{
    card::{color::Color, value::Value, Card},
    error::GameError,
    rule::Rule,
};

const MAX_TURNS: u32 = 5;

pub struct Game {
    rules: Box<dyn Rule>,
    deck: Vec<Card>,
    turn: u32,
    last_turn_score: i32,
}

impl Game {
    pub fn new(rules: Box<dyn Rule>) -> Game {
        Game {
            rules,
            deck: vec![],
            turn: 0,
            last_turn_score: 0,
        }
    }

    pub fn pick_cards(&mut self) -> Result<(Card, Card), GameError> {
        if self.deck.len() < 2 {
            return Err(GameError::EmptyDeck);
        }
        if self.turn >= MAX_TURNS {
            return Err(GameError::GameIsOver);
        }
        self.turn += 1;
        let card1 = self.deck.pop().unwrap();
        let card2 = self.deck.pop().unwrap();
        Ok((card1, card2))
    }

    pub fn get_rules(&self) -> &dyn Rule {
        self.rules.as_ref()
    }

    pub fn reset_deck(&mut self) {
        let mut deck = Vec::<Card>::new();
        let colors = vec![Color::Spade, Color::Heart, Color::Club, Color::Diamond];
        let values = vec![
            Value::Number(7),
            Value::Number(8),
            Value::Number(9),
            Value::Number(10),
            Value::Jack,
            Value::Queen,
            Value::King,
            Value::Ace,
        ];
        for color in colors {
            for value in &values {
                deck.push(Card::new(color, *value));
            }
        }
        self.deck = deck;
        self.last_turn_score = 0;
        self.turn = 0;
    }

    pub fn shuffle_deck(&mut self) {
        use rand::seq::SliceRandom;
        self.deck.shuffle(&mut rand::thread_rng());
    }

    pub fn get_remaining_turns(&self) -> u32 {
        MAX_TURNS - self.turn
    }

    pub fn get_last_turn_score(&self) -> i32 {
        self.last_turn_score
    }

    pub fn set_last_turn_score(&mut self, score: i32) {
        self.last_turn_score = score;
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::rules;

    use super::*;

    #[test]
    fn deck_size() {
        let mut game = Game::new(Box::new(rules()));
        game.reset_deck();
        assert_eq!(game.deck.len(), 32);
    }

    #[test]
    fn deck_shuffle() {
        let mut game = Game::new(Box::new(rules()));
        game.reset_deck();
        let deck = game.deck.clone();
        game.shuffle_deck();
        assert_ne!(game.deck, deck);
    }
}
