# RISC-0 Ting-Tong

Welcome to our hackathon project! We've developed a simple, incomplete information game based on the RISC-0. This game is a fun way to challenge your friends and test your guessing abilities.

## Quick Start

First, make sure [rustup](https://rustup.rs) is installed. This project uses a [nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) version of [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html). The [`rust-toolchain`](rust-toolchain) file will be used by `cargo` to automatically install the correct version. To start the game, run the following command:

```
cargo run -r
```

## Overview

In this game, each player has 2 thumbs, and the total number of thumbs at the beginning is equal to 2 times the number of players. The goal is to be the first player to have zero thumbs by correctly guessing the number of lifted thumbs in each round.

## Rules

1. Each player commits to a choice of lifting 0, 1, or 2 thumbs.
2. After all players have made their choices, they simultaneously commit to their guesses on the total number of lifted thumbs.
3. Players who guess correctly will hide one thumb.
4. The game continues in rounds until one player has hidden both thumbs, making them the winner.

## How to Play

To play the RISC-0 Ting-Tong, follow these steps:

1. Each player must secretly choose how many thumbs they will lift (0, 1, or 2) and submit their choice.
2. After all, choices have been submitted, each player must guess the total number of lifted thumbs and submit their guess.
3. Players who guess correctly will hide one thumb.
4. Repeat steps 1-3 until a player has hidden both of their thumbs and wins the game.

Enjoy playing the RISC-0 Ting-Tong and may the best guesser win!
