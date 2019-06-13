use std::iter::Iterator;

use crate::Cell;

#[derive(Default, Clone)]
pub struct Pattern {
    instructions: Vec<Cell>,
}

impl Pattern {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn generate(&mut self, length: usize) {
        self.clear();
        for _ in 0 .. length {
            self.instructions.push(Cell::random());
        }
    }

    pub fn generate_append(&mut self, incr: u32) {
        for _ in 0 .. incr {
            self.instructions.push(Cell::random());
        }
    }

    pub fn clear(&mut self) {
        self.instructions.clear();
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn iter(&self) -> PatternIter {
        PatternIter::new(self.instructions.clone())
    }
}

pub struct PatternIter {
    instructions: Vec<Cell>,
    index:        usize,
}

impl PatternIter {
    pub fn new(instructions: Vec<Cell>) -> Self {
        Self {
            instructions,
            index: 0,
        }
    }
}

impl Iterator for PatternIter {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cell) = self.instructions.get(self.index) {
            self.index += 1;
            Some(cell.clone())
        } else {
            None
        }
    }
}
