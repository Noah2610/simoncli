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
        use std::io::{stdin, BufRead};

        let mut pattern_iter = self.pattern.iter();
        let mut guesses = Vec::<Cell>::new();
        let mut total_guesses = 0;
        let mut msg_prefix = "";

        while total_guesses < self.pattern.len() {
            print(format!("{}> ", msg_prefix));
            flush()?;
            msg_prefix = "";

            let mut input = String::new();
            if let Err(e) = stdin().lock().read_line(&mut input) {
                return Err(e.to_string());
            };
            let input = input.trim();

            // Validate inputs
            if input
                .chars()
                .all(|c| self.controls.iter().any(|control| control.key.is(&c)))
            {
                guesses.append(
                    &mut input
                        .chars()
                        .map(|c| {
                            self.controls
                                .iter()
                                .find_map(|control| {
                                    if control.key.is(&c) {
                                        Some(control.cell.clone())
                                    } else {
                                        None
                                    }
                                })
                                .expect(&format!(
                                    "Given input key '{}' should be valid, \
                                     because validation happened above",
                                    c
                                ))
                        })
                        .collect(),
                );
            } else {
                msg_prefix = "invalid ";
            }

            if input.is_empty() {
                msg_prefix = "type your guess(es) ";
            }

            for &guess in guesses.iter() {
                if let Some(cell) = pattern_iter.next() {
                    if guess == cell {
                        total_guesses += 1;
                        self.score += 1;
                    } else {
                        self.game_over()?;
                        return Ok(());
                    }
                } else {
                    self.game_over()?;
                    return Ok(());
                }
            }
            guesses.clear();
        }

        if pattern_iter.next().is_some() {
            self.game_over()?;
            return Ok(());
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
