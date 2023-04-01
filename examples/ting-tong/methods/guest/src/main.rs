#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};
use ting_tong_core::{GameState, Guess, Score};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let server_guess: Guess = env::read();
    let player_guess: Guess = env::read();
    let mut score: Score = env::read();

    if player_guess.secret_guess == 5 && player_guess.secret_choice == 5 {
        let server_hash =
            *Impl::hash_bytes(&[server_guess.secret_guess, server_guess.secret_choice]);

        let game_state = GameState {
            server_hash,
            server_count: score.server_score,
            player_count: score.player_score,
        };

        env::commit(&game_state);
    } else {
        let correct = server_guess.secret_choice + player_guess.secret_choice;

        let player_result = player_guess.secret_guess == correct;
        let server_result = server_guess.secret_guess == correct;

        if server_result {
            score.server_score -= 1;
        }
        if player_result {
            score.player_score -= 1;
        }

        let server_hash =
            *Impl::hash_bytes(&[server_guess.secret_guess, server_guess.secret_choice]);
        // let server_hash = *Impl::hash_bytes(&to_vec(&server_guess).unwrap());
        let game_state = GameState {
            server_hash,
            server_count: score.server_score,
            player_count: score.player_score,
        };

        env::commit(&game_state);
    }
}
