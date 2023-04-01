#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};
use ting_tong_core::{GameState, Guess};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let mut server_count = 2;
    let mut player_count = 2;

    let server_guess: Guess = env::read();
    let player_guess: Guess = env::read();

    if player_guess.secret_guess == 5 && player_guess.secret_choice == 5 {
        let server_hash = *Impl::hash_bytes(&[server_guess.secret_guess, server_guess.secret_choice]);
        
        let game_state = GameState {
            server_hash,
            server_count,
            player_count,
        };

        env::commit(&game_state);
    } else {
        let correct = server_guess.secret_choice + player_guess.secret_choice;
        
        let player_result = player_guess.secret_guess == correct;
        let server_result = server_guess.secret_guess == correct;

        if server_result {
            server_count -= 1;
        }
        if player_result {
            player_count -= 1;
        }

        let server_hash = *Impl::hash_bytes(&[server_guess.secret_guess, server_guess.secret_choice]);
        // let server_hash = *Impl::hash_bytes(&to_vec(&server_guess).unwrap());
        let game_state = GameState {
            server_hash,
            server_count,
            player_count,
        };

        env::commit(&game_state);
    }
}
