#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};
use ting_tong_core::{GameState, Play, Score};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let server_play: Play = env::read();
    let player_play: Play = env::read();
    // actual game score (from previous rounds)
    let mut score: Score = env::read();

    let thumbs_up = server_play.secret_choice + player_play.secret_choice;

    if server_play.secret_guess == thumbs_up {
        score.server_score -= 1;
    }
    if player_play.secret_guess == thumbs_up {
        score.player_score -= 1;
    }

    let server_hash = *Impl::hash_bytes(&[server_play.secret_guess, server_play.secret_choice]);
    let game_state = GameState {
        server_hash,
        server_score: score.server_score,
        player_score: score.player_score,
    };

    env::commit(&game_state);
}
