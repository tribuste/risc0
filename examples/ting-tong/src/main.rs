use std::io::{stdin, stdout, Write};

use rand::Rng;
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    sha::Digest,
    Prover, Receipt,
};
use throbber::Throbber;
use ting_tong_core::{GameState, Play, Score};
use ting_tong_methods::{TING_TONG_ELF, TING_TONG_ID};

struct Server {
    secret: Play,
}

impl Server {
    pub fn new_play(score: &Score) -> Self {
        Server {
            secret: Play {
                secret_choice: rand::thread_rng().gen_range(0..=score.server_score),
                secret_guess: rand::thread_rng().gen_range(0..=(score.player_score + score.player_score)),
            },
        }
    }

    pub fn get_secret(&self, score: &Score) -> Digest {
        let dummy_guess = Play::default();

        let receipt = self.eval_round(dummy_guess, score);
        let game_state: GameState = from_slice(&receipt.journal).unwrap();

        game_state.server_hash
    }

    pub fn eval_round(&self, player_guess: Play, score: &Score) -> Receipt {
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
            server_score: game_state.server_score,
            player_score: game_state.player_score,
        };
    }
}

fn read_stdin_guess(score: &Score) -> Play {
    let mut line = String::new();
    let mut guess = Play {
        secret_choice: 0,
        secret_guess: 0,
    };

    loop {
        println!("\nThumbs up!:");
        let _ = stdout().flush();
        stdin().read_line(&mut line).unwrap();
        line.pop(); // remove trailing newline

        match line.parse::<u8>() {
            Ok(res) => {
                if res <= score.player_score {
                    guess.secret_choice = res;
                    break;
                } else {
                    println!("You don't have enought thumbs!! ;D");
                    line.clear();
                    continue;
                }
            }
            Err(_) => {
                println!("Must by a number!!");
                line.clear();
                continue;
            }
        }
    }
    line.clear();
    loop {
        println!("What is your guess? How many thumbs will be up!?:");
        let _ = stdout().flush();
        stdin().read_line(&mut line).unwrap();
        line.pop(); // remove trailing newline

        match line.parse::<u8>() {
            Ok(res) => {
                if res <= (score.player_score + score.server_score) {
                    guess.secret_guess = res;
                    break;
                } else {
                    println!("Players have only {} thumbs left in the game!!", (score.player_score + score.server_score));
                    line.clear();
                    continue;
                }
            }
            Err(_) => {
                println!("Must by a number!!");
                line.clear();
                continue;
            }
        }
    }

    guess
}

fn game_is_won(score: &Score) -> bool {
    // print the actual score of the game
    println!(
        "\n\nServer hands: {}\tPlayer hands: {}",
        score.server_score, score.player_score
    );

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
    let mut proof_throbber = Throbber::new().message("Generating proof & Verification...".to_string());
    let mut commit_throbber = Throbber::new().message("Server commiting to the guess...".to_string());
    let mut round = 1;

    println!("Let's play TING TONG!!");

    let mut game_won = false;
    let mut score = Score {
        server_score: 2,
        player_score: 2,
    };

    while game_won == false {
        print!("\x1b[43m");
        println!("\nRound {round}");
        print!("\x1b[0m");
        let server_guess = Server::new_play(&score);
        commit_throbber.start();
        let player = Player {
            hash: server_guess.get_secret(&score),
        };
        commit_throbber.finish();

        let player_guess = read_stdin_guess(&score);

        proof_throbber.start();
        let receipt = server_guess.eval_round(player_guess, &score);
        score = player.check_receipt(receipt);
        proof_throbber.finish();

        game_won = game_is_won(&score);
        round+=1;
    }
}
