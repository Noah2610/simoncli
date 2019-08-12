# Simon CLI
A simple, little CLI [simon game][simon-wikipedia].  
Written in Rust; the only crate dependency is `rand` for randomization.

__Asciicast__  
[![asciicast](https://asciinema.org/a/261989.svg)](https://asciinema.org/a/261989)

## Installation
### From [crates.io]
```
cargo install simoncli
```

### From source
You will need `cargo` to compile from source, which is shipped with Rust.  
Run `cargo run` from within the project's root to compile and run the game.  
You can also install from source by running the following from within the project's root:
```
cargo install --path .
```
Now you should have `simoncli` available from your shell.

## Usage
Once installed, simply run the game with `simoncli` (or `cargo run`)  
and the game will start.  
Remember the order in which numbers show up in the squares,  
then repeat the pattern by typing the associated keys in the same order.

- `w` for __top-left__
- `e` for __top-right__
- `s` for __bottom-left__
- `d` for __bottom-right__

You can type out the whole pattern of keys in the same line and press enter to submit,  
or you can input and submit the keys one-at-a-time.

If you mess-up, the game ends and your score is displayed.  
You get one score point for every correct key in a pattern.  
Your highscore is not saved; this might be a cool feature to have in the future.

[simon-wikipedia]: https://en.wikipedia.org/wiki/Simon_(game)
[crates.io]:       https://crates.io/crates/simoncli
