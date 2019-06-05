use std::time::Duration;

use crate::controls::Controls;
use crate::Board;
use crate::Cell;
use crate::Pattern;
use crate::Res;
use crate::{cmd, flush, print, sleep_ms};

const INITIAL_PATTERN_LENGTH: usize = 2;

pub struct Game {
    is_running: bool,
    board:      Board,
    pattern:    Pattern,
    controls:   Controls,
    score:      u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            is_running: false,
            board:      Board::new(),
            pattern:    Pattern::empty(),
            controls:   Controls::new(),
            score:      0,
        }
    }

    pub fn run(&mut self) -> Res<String> {
        cmd("clear")?;

        self.is_running = true;
        self.pattern.generate(INITIAL_PATTERN_LENGTH);

        while self.is_running {
            self.update()?;
        }
        Ok(())
    }

    fn update(&mut self) -> Res<String> {
        cmd("clear")?;
        flush()?;

        // Append to pattern
        self.pattern.generate_append(1);

        // Print current pattern
        self.board.print_pattern(&self.pattern)?;

        // Print letters for cells
        self.board.print_keys(&self.controls)?;

        // Read from stdin
        self.read_input()?;

        Ok(())
    }

    fn read_input(&mut self) -> Res<String> {
        use std::io::{stdin, BufRead, Read};

        for cell in self.pattern.iter() {
            let mut should_read_input = true;
            let mut guess = None;

            while guess.is_none() {
                print("> ");
                flush()?;

                let mut input = String::new();
                stdin().lock().read_line(&mut input);
                let input = input.trim();

                guess = self.controls.iter().find_map(|control| {
                    if control.key.is(&input) {
                        Some(control.cell.clone())
                    } else {
                        None
                    }
                });
            }

            if let Some(guess) = guess {
                if guess == cell {
                    self.score += 1;
                } else {
                    self.game_over()?;
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn game_over(&self) -> Res<String> {
        use std::process::exit;

        cmd("clear")?;
        print(format!("Game Over!\nScore: {}\n", self.score));
        flush()?;
        exit(0);
        Ok(())
    }
}
