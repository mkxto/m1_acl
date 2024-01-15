use crate::{card::Card, error::GameError, game::Game};

/// ## Player
///
/// A structure that represents a player in the game.
#[derive(Debug)]
pub struct Player {
    name: String,
    score: i32,
    hand: Vec<Card>,
}

impl Player {
    /// ## Create a new Player given a name.
    ///
    /// Returns a new Player with the given name, a score of 0 and an empty hand.
    pub fn new(name: String) -> Result<Player, GameError> {
        if name.is_empty() {
            return Err(GameError::EmptyPlayerName);
        }
        Ok(Player {
            name,
            score: 0,
            hand: Vec::<Card>::new(),
        })
    }

    /// ## Set the name of the Player.
    ///
    /// Returns an error if the name is empty.
    pub fn set_name(&mut self, name: String) -> Result<(), GameError> {
        if name.is_empty() {
            Err(GameError::EmptyPlayerName)
        } else {
            self.name = name;
            Ok(())
        }
    }

    /// ## Get the name of the Player.
    ///
    /// Returns an immutable reference to the name of the Player.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// ## Get the score of the Player.
    ///
    /// Returns the score of the Player.
    pub fn get_score(&self) -> i32 {
        self.score
    }

    /// ## Get the hand of the Player.
    ///
    /// Returns an immutable reference to the hand of the Player.
    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    /// ## Pick cards from the deck.
    ///
    /// Picks two cards from the deck and adds them to the hand of the Player.
    ///
    /// Returns an error if the cards cannot be picked.
    pub fn pick_cards(&mut self, game: &mut Game) -> Result<i32, GameError> {
        let cards = game.pick_cards()?;
        let points = match game.get_rules().execute((&cards.0, &cards.1)) {
            Ok(score) => score,
            Err(GameError::CannotHandleCards) => 0,
            Err(e) => return Err(e),
        };
        self.score += points;
        game.set_last_turn_score(points);
        self.hand.push(cards.0);
        self.hand.push(cards.1);
        Ok(points)
    }

    /// ## Reset the score of the Player.
    ///
    /// Resets the score of the Player to 0.
    pub fn reset_score(&mut self) {
        self.score = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::rules;

    #[test]
    fn player_new() {
        let player = Player::new("Player".to_string()).unwrap();
        assert_eq!(player.get_name(), "Player");
        assert_eq!(player.get_score(), 0);
        assert_eq!(player.get_hand().len(), 0);
    }

    #[test]
    fn player_set_name() {
        let mut player = Player::new("Player".to_string()).unwrap();
        assert_eq!(
            player.set_name("".to_string()),
            Err(GameError::EmptyPlayerName)
        );
        assert_eq!(player.set_name("Player1".to_string()), Ok(()));
        assert_eq!(player.get_name(), "Player1");
    }

    #[test]
    fn player_get_name() {
        let player = Player::new("Player".to_string()).unwrap();
        assert_eq!(player.get_name(), "Player");
    }

    #[test]
    fn player_get_score() {
        let player = Player::new("Player".to_string()).unwrap();
        assert_eq!(player.get_score(), 0);
    }

    #[test]
    fn player_get_empty_hand() {
        let player = Player::new("Player".to_string()).unwrap();
        assert!(player.get_hand().is_empty());
    }

    #[test]
    fn player_pick_cards() {
        let rules = rules();
        let mut game = Game::new(&rules);
        game.reset_deck();
        let mut player = Player::new("Player".to_string()).unwrap();
        assert_eq!(player.pick_cards(&mut game), Ok(0));
        assert_eq!(player.get_hand().len(), 2);
        assert_eq!(player.pick_cards(&mut game), Ok(0));
        assert_eq!(player.get_hand().len(), 4);
    }
}
