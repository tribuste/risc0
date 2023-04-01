use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Guess {
    pub secret_guess: u8,
    pub secret_choice: u8,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Score {
    pub server_score: u8,
    pub player_score: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GameState {
    pub server_hash: Digest,
    pub server_count: u8,
    pub player_count: u8,
}
