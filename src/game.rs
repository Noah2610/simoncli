use crate::cmd;
use crate::print;
use crate::Board;
use crate::Cell;
use crate::Res;

pub struct Game {
    is_running:     bool,
    board:          Board,
    pattern_length: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            is_running:     false,
            board:          Board::new(),
            pattern_length: 2,
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
        use std::thread::sleep;
        use std::time::Duration;

        print("New round!\n");
        sleep(Duration::from_millis(500));

        // Print current pattern
        self.board.print_pattern()?;

        // Sleep
        sleep(Duration::from_millis(1000));

        // Generate new pattern
        self.pattern_length += 1;
        self.board.generate_pattern(self.pattern_length);

        // Redraw the screen
        cmd("clear")?;

        Ok(())
    }
}
