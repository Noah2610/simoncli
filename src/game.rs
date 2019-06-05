use std::thread::sleep;
use std::time::Duration;

use crate::controls::Controls;
use crate::Board;
use crate::Cell;
use crate::Pattern;
use crate::Res;
use crate::{cmd, print};

pub struct Game {
    is_running:     bool,
    board:          Board,
    pattern:        Pattern,
    pattern_length: usize,
    controls:       Controls,
}

impl Game {
    pub fn new() -> Self {
        Self {
            is_running:     false,
            board:          Board::new(),
            pattern:        Pattern::empty(),
            pattern_length: 2,
            controls:       Controls::new(),
        }
    }

    pub fn run(&mut self) -> Res<String> {
        cmd("clear")?;

        self.is_running = true;

        while self.is_running {
            self.update()?;
        }
        Ok(())
    }

    fn update(&mut self) -> Res<String> {
        cmd("clear")?;

        // Generate new pattern
        self.pattern.generate(self.pattern_length);
        self.pattern_length += 1;

        print("New round!\n");
        sleep(Duration::from_millis(1000));

        // Print current pattern
        self.board.print_pattern(&self.pattern)?;

        sleep(Duration::from_millis(500));

        // Print letters for cells
        self.board.print_keys(&self.controls)?;

        // Read from stdin
        self.read_input();

        sleep(Duration::from_millis(1000));

        Ok(())
    }

    fn read_input(&mut self) {
        for index in (0 .. self.pattern_length) {}
    }
}
