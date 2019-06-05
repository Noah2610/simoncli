use std::time::Duration;

use crate::cmd;
use crate::print;
use crate::Cell;
use crate::Pattern;
use crate::Res;

const DELAY_MS: u64 = 500;
const SPACE: &str = " ";
const NEWLINE: &str = "\n";
const EMPTY: &str = "□";
const FILLED: &str = "█";

pub struct Board {
    pub pattern:     Option<Pattern>,
    current_pattern: usize,

    rows:          Vec<Vec<Cell>>,
    padding:       [usize; 2],
    inner_padding: [usize; 2],

    delay: Duration,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pattern:         None,
            current_pattern: 0,

            rows:          vec![vec![Cell::TopLeft, Cell::TopRight], vec![
                Cell::BotLeft,
                Cell::BotRight,
            ]],
            padding:       [1, 2],
            inner_padding: [1, 2],

            delay: Duration::from_millis(DELAY_MS),
        }
    }

    pub fn print_pattern(&mut self) -> Res<String> {
        use std::io::{stdout, Write};
        use std::thread::sleep;

        if let Some(pattern) = &self.pattern {
            for (pat_index, pat_cell) in pattern.iter().enumerate() {
                cmd("clear")?;

                for _ in (0 .. self.padding[0]) {
                    print(NEWLINE);
                }

                for (row, cells) in self.rows.iter().enumerate() {
                    for _ in (0 .. self.padding[1]) {
                        print(SPACE);
                    }

                    for cell in cells {
                        if cell == &pat_cell {
                            // print(FILLED);
                            print(pat_index + 1);
                        } else {
                            print(EMPTY);
                        }
                        for _ in (0 .. self.inner_padding[0]) {
                            print(SPACE);
                        }
                    }

                    print(NEWLINE);
                }

                if stdout().flush().is_err() {
                    return Err("Couldn't flush stdout".into());
                }
                sleep(self.delay);
            }
        }

        if self.pattern.is_some() {
            self.pattern = None;
        }

        Ok(())
    }

    pub fn generate_pattern(&mut self, length: usize) {
        if let Some(pattern) = self.pattern.as_mut() {
            pattern.generate(length);
        } else {
            let mut pattern = Pattern::empty();
            pattern.generate(length);
            self.pattern = Some(pattern);
        }
    }
}
