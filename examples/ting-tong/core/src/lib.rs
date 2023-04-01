use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

// data accompanied with a one round of the game
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Play {
    pub secret_guess: u8,  // guess of how many thumbs will be up in total
    pub secret_choice: u8, // choice of how many thumbs you will raise up
}

// score represents how many hands a player has in the game
// after each succesfull guess a player pull of one hand from the game
// winner is a player with no hands in the game
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Score {
    pub server_score: u8, // "hands" of server
    pub player_score: u8, // "hands" of player
}

// data written to the journal
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GameState {
    pub server_hash: Digest, // commitment of the server
    pub server_score: u8,    // server score after a play
    pub player_score: u8,    // player score after a play
}
