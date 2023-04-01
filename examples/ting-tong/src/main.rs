// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::{stdin, stdout, Write};

use risc0_zkvm::{
    serde::{from_slice, to_vec},
    sha::Digest,
    Prover, Receipt,
};
use ting_tong_core::{GameState, Guess, Score};
use ting_tong_methods::{TING_TONG_ELF, TING_TONG_ID};

use rand::Rng;

struct HonestServer {
    secret: Guess,
}

impl HonestServer {
    pub fn new_guess() -> Self {
        let choice = rand::thread_rng().gen_range(0..3);
        let guess = rand::thread_rng().gen_range(0..5);
        HonestServer {
            secret: Guess {
                secret_choice: choice,
                secret_guess: guess,
            },
        }
    }

    pub fn get_secret(&self) -> Digest {
        let dummy_guess = Guess {
            secret_choice: 5,
            secret_guess: 5,
        };

        let score = Score::default();
        let receipt = self.eval_round(dummy_guess, &score);
        let game_state: GameState = from_slice(&receipt.journal).unwrap();
        game_state.server_hash
    }

    pub fn eval_round(&self, player_guess: Guess, score: &Score) -> Receipt {
        let mut prover = Prover::new(TING_TONG_ELF).expect("failed to construct prover");

        prover.add_input_u32_slice(to_vec(&self.secret).unwrap().as_slice());
        prover.add_input_u32_slice(to_vec(&player_guess).unwrap().as_slice());
        prover.add_input_u32_slice(to_vec(&score).unwrap().as_slice());

        return prover.run().unwrap();
    }
}

struct Player {
    pub hash: Digest,
}

impl Player {
    pub fn check_receipt(&self, receipt: Receipt) -> Score {
        receipt
            .verify(&TING_TONG_ID)
            .expect("receipt verification failed");

        let game_state: GameState = from_slice(&receipt.journal).unwrap();
        if game_state.server_hash != self.hash {
            panic!("The hash mismatched, so the server cheated!");
        }

        return Score {
            server_score: game_state.server_count,
            player_score: game_state.player_count,
        }
    }
}

fn read_stdin_guess() -> Guess {
    let mut line = String::new();
    let mut guess = Guess {
        secret_choice: 0,
        secret_guess: 0,
    };

    loop {
        print!("Thumbs up!:");
        let _ = stdout().flush();
        stdin().read_line(&mut line).unwrap();
        line.pop(); // remove trailing newline

        match line.parse::<u8>() {
            Ok(res) => {
                if res < 3 {
                    guess.secret_choice = res;
                    break;
                } else {
                    println!("WTF!? You have only 2 thumbs!!\n");
                    line.clear();
                    continue;
                }
            }
            Err(_) => {
                println!("Must by a number!!\n");
                line.clear();
                continue;
            }
        }
    }
    line.clear();
    loop {
        print!("What is your guess? How many thumbs will be up!?:");
        let _ = stdout().flush();
        stdin().read_line(&mut line).unwrap();
        line.pop(); // remove trailing newline

        match line.parse::<u8>() {
            Ok(res) => {
                if res < 5 {
                    guess.secret_guess = res;
                    break;
                } else {
                    println!("2 players have only 4 thumbs in total!!\n");
                    line.clear();
                    continue;
                }
            }
            Err(_) => {
                println!("Must by a number!!\n");
                line.clear();
                continue;
            }
        }
    }

    guess
}

fn game_is_won(score: &Score) -> bool {
    if score.server_score == 0 {
        println!("You lost!!");
        true
    } else if score.player_score == 0 {
        println!("You won!!");
        true
    } else {
        false
    }
}

fn main() {
    println!("Let's play TING TONG!!");

    let mut game_won = false;
    let mut score = Score {
        server_score: 2,
        player_score: 2,
    };

    while game_won == false {
        let server_guess = HonestServer::new_guess();
        let player = Player {
            hash: server_guess.get_secret(),
        };

        let player_guess = read_stdin_guess();
        let receipt = server_guess.eval_round(player_guess, &score);
        score = player.check_receipt(receipt);

        game_won = game_is_won(&score);
    }
}