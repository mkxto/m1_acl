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
    game: game::Game<'static>,
    player: player::Player,
    rules: &'static dyn Rule,
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new(username: String) -> Self {
        let rules = rules_static();
        let game = game::Game::new(&rules);
        let player = player::Player::new(username).unwrap();
        Self {
            game,
            player,
            rules: &rules,
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.game.reset_deck();
        self.game.shuffle_deck();
        self.player.reset_score();
    }

    #[wasm_bindgen]
    pub fn pick(&mut self) -> Result<(), JsValue> {
        self.player.pick_cards(&mut self.game).unwrap();
        Ok(())
    }

    #[wasm_bindgen]
    pub fn get_score(&self) -> i32 {
        self.player.get_score()
    }
}

#[wasm_bindgen]
pub fn play_whole_game(username: String) -> i32 {
    let mut state = GameState::new(username);
    for _ in 0..5 {
        state.pick().unwrap();
    }
    state.get_score()
}
