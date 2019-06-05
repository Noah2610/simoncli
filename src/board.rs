use std::time::Duration;

use crate::controls::Controls;
use crate::Cell;
use crate::Pattern;
use crate::Res;
use crate::{cmd, flush, print};

const DELAY_MS: u64 = 500;
const SPACE: &str = " ";
const NEWLINE: &str = "\n";
const EMPTY: &str = "□";
const FILLED: &str = "█";

pub struct Board {
    rows:          Vec<Vec<Cell>>,
    padding:       [usize; 2],
    inner_padding: [usize; 2],

    delay: Duration,
}

impl Board {
    pub fn new() -> Self {
        Self {
            rows:          vec![vec![Cell::TopLeft, Cell::TopRight], vec![
                Cell::BotLeft,
                Cell::BotRight,
            ]],
            padding:       [1, 2],
            inner_padding: [1, 2],

            delay: Duration::from_millis(DELAY_MS),
        }
    }

    pub fn print_pattern(&mut self, pattern: &Pattern) -> Res<String> {
        use std::io::{stdout, Write};
        use std::thread::sleep;

        for (pat_index, pat_cell) in pattern.iter().enumerate() {
            cmd("clear")?;

            for _ in 0 .. self.padding[0] {
                print(NEWLINE);
            }

            for cells in self.rows.iter() {
                for _ in 0 .. self.padding[1] {
                    print(SPACE);
                }

                for cell in cells {
                    if cell == &pat_cell {
                        print(pat_index + 1);
                    } else {
                        print(EMPTY);
                    }
                    for _ in 0 .. self.inner_padding[1] {
                        print(SPACE);
                    }
                }

                for _ in 0 .. self.inner_padding[0] {
                    print(NEWLINE);
                }

                print(NEWLINE);
            }

            flush()?;
            sleep(self.delay);
        }

        Ok(())
    }

    pub fn print_keys(&self, controls: &Controls) -> Res<String> {
        cmd("clear")?;

        for _ in (0 .. self.padding[0]) {
            print(NEWLINE);
        }

        for (row, cells) in self.rows.iter().enumerate() {
            for _ in (0 .. self.padding[1]) {
                print(SPACE);
            }

            for cell in cells {
                if let Some(key) = controls.get_key_by_cell(cell) {
                    print(key.to_string());
                } else {
                    print(EMPTY);
                }
                for _ in (0 .. self.inner_padding[1]) {
                    print(SPACE);
                }
            }

            for _ in 0 .. self.inner_padding[0] {
                print(NEWLINE);
            }

            print(NEWLINE);
        }

        flush()?;
        Ok(())
    }
}
