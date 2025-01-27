# RISC-0 Ting-Tong

Welcome to our hackathon project! We've developed a simple, incomplete information game based on the RISC-0. This game is a fun way to challenge your friends and test your guessing abilities.

## Quick Start

First, make sure [rustup](https://rustup.rs) is installed. This project uses a [nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) version of [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html). The [`rust-toolchain`](rust-toolchain) file will be used by `cargo` to automatically install the correct version. To start the game, run the following command:

```
cargo run -r
```

## Overview

In this game, each player uses his/her 2 hands (i.e. 2 thumbs). The total number of thumbs in the game equals 2 * `number_of_players`. The goal is to be the first player to have zero hands by correctly guessing the number of lifted thumbs in each round. After each correct guess you can pull out one
hand from the game.

## Rules

1. At every round each player (i.e. server and player) commits to a choice by lifting 0, 1, or 2 thumbs.
2. They simultaneously commit to their guesses where they are guessing the **total** number of lifted thumbs.
3. Players who guess correctly will pull off one hand (note: in the next round the number of thumbs decreases).
4. The game continues until someone pull of both hands from the game, making him/her the winner.

## Ensuring fair play
The player ensures that server isn't cheating using the `check_receipt` function. This function verifies that the
receipt is valid and was generated by the correct binary file.
Then the `check_receipt` function checks that the has in 
the `journal` match the hash of the secret guess provided 
by the server at the beginning of each round.

## How to Play

To play the RISC-0 Ting-Tong, follow these steps:

1. Run `cargo run -r`
2. Wait for server to commit to it's guess and choice.
3. Enter you choice and guess.
4. Wait for verification and evaluation of a round. 
5. After verification and evaluation the score of a round will be printed out and if you or server (or both) guessed the correct number of lifted thumbs in a round your score is substracted by 1 (simulating pulling of a hand from the game).
6. Process repeats from the 2. point until some of you reach `score=0`

Enjoy playing the RISC-0 Ting-Tong and may the best "guesser" win!
