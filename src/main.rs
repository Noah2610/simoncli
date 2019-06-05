mod board;
mod cell;
mod game;
mod pattern;

use std::fmt::Debug;
use std::fmt::Display;

use board::Board;
use cell::Cell;
use game::Game;
use pattern::Pattern;

type Res<E: Debug> = Result<(), E>;

fn cmd<T: ToString>(cmd: T) -> Res<String> {
    use std::process::Command;
    let cmd = cmd.to_string();
    if !Command::new(&cmd).status().unwrap().success() {
        Err(format!("Command '{}' failed:", cmd))
    } else {
        Ok(())
    }
}

fn print<T: Display>(s: T) {
    print!("{}", s);
}

fn main() {
    let mut game = Game::new();

    if let Err(err) = game.run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
