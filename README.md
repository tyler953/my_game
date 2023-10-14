# Overview

This is a program I created to start learning to use Rust. I created a simple Tetris clone to explore may different aspects of the language as well as to explore some aspects of game design and development.
I was able to learn a lot about the applications of Rust and its syntax. I was surprised by how similar it is to other languages I've used while still having a number of features that make it a little more
strict in its usage. The program itself creates a small window with a 30x30 grid of cells in which a single block will slowly fall fall to the bottom. After the reaching the bottom, it will start a new block 
falling from the top again. It has built in collision detection to prevent blocks from leaving the grid and from overlapping with other blocks.

[Software Demo Video](https://youtu.be/zb7coPbJ_Go)

# Development Environment

For this project I used the Rust programming language, VSCode with some of its built in Rust support, and primarily the Rust library 'ggez'.

# Useful Websites

- [Exercism](https://exercism.org/tracks/rust)
- [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-02-match.html)
- [ggez Documentation](https://docs.rs/ggez/latest/ggez/)
- [Pong by yotam5](https://github.com/yotam5/pong_ggez/blob/main/src/game/game_state.rs#L37)

# Future Work

- Turn blocks into multi cell Tetrominoes
- Enable the user to rotate the Tetrominoes
- Implement line clearing, scoring, and game over
- Implement a ui/start menu
