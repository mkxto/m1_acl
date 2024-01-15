pub mod card;
pub mod error;
pub mod game;
pub mod player;
pub mod rule;
pub mod rules;
pub mod saver;

use rule::Rule;
use wasm_bindgen::prelude::*;

use crate::rules::rules_static;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct GameState {
    game: game::Game,
    player: player::Player,
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new(username: String) -> Self {
        let player = player::Player::new(username).unwrap();
        Self {
            game: game::Game::new(Box::new(rules_static())),
            player,
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.game.reset_deck();
        self.game.shuffle_deck();
        self.player.reset_score();
    }

    #[wasm_bindgen]
    pub fn pick(&mut self) -> Result<(), String> {
        match self.player.pick_cards(&mut self.game) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }

    #[wasm_bindgen]
    pub fn get_score(&self) -> i32 {
        self.player.get_score()
    }

    #[wasm_bindgen]
    pub fn get_last_score(&self) -> i32 {
        self.game.get_last_turn_score()
    }

    #[wasm_bindgen]
    pub fn get_remaining_turns(&self) -> u32 {
        self.game.get_remaining_turns()
    }

    #[wasm_bindgen]
    pub fn get_last_two_cards_id(&self) -> Vec<String> {
        let hand = self.player.get_hand();
        let card1 = hand[hand.len() - 1].get_id();
        let card2 = hand[hand.len() - 2].get_id();
        vec![card1, card2]
    }

    #[wasm_bindgen]
    pub fn save_game(&mut self) {
        use crate::saver::Savable;
        log("Saving game...");
        let mut saver = saver::WebLocalStorageSaver::default();
        saver.save_score(self.player.get_name(), self.player.get_score());
        log("Game saved from Rust");
    }
}
