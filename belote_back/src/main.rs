pub mod card;
pub mod error;
pub mod game;
pub mod player;
pub mod rule;
pub mod rules;
pub mod saver;

use crate::game::Game;
use crate::player::Player;

use rules::rules;

fn main() {
    let rules = rules();
    let mut game = Game::new(&rules);
    let mut player = Player::new("Player".to_string()).unwrap();
    let mut min_score = 0;
    let mut max_score = 0;
    for _ in 0..10000000 {
        game.reset_deck();
        game.shuffle_deck();
        player.reset_score();
        for _ in 0..5 {
            player.pick_cards(&mut game).unwrap();
        }
        let score = player.get_score();
        if score < min_score {
            min_score = score;
            println!("New min score: {}", min_score);
        }
        if score > max_score {
            max_score = score;
            println!("New max score: {}", max_score);
        }
    }
}
