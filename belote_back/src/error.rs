use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum GameError {
    EmptyPlayerName,
    InvalidPlayerName,
    CannotHandleCards,
    EmptyDeck,
    GameIsOver,
}

impl std::error::Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::InvalidPlayerName => write!(f, "Player's name is invalid."),
            GameError::EmptyPlayerName => write!(f, "Player's name cannot be empty."),
            GameError::CannotHandleCards => write!(f, "No rule was able to handle the cards."),
            GameError::EmptyDeck => write!(f, "The deck is empty."),
            GameError::GameIsOver => write!(f, "The game is over."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_eq() {
        assert_eq!(
            format!("{}", GameError::EmptyPlayerName),
            "Player's name cannot be empty."
        );
    }
}
