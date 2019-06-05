use std::collections::HashMap;

use crate::Cell;

#[derive(PartialEq, Clone)]
pub enum Key {
    W,
    E,
    S,
    D,
}

impl Key {
    pub fn is<T: ToString>(&self, s: T) -> bool {
        self.to_string().to_lowercase() == s.to_string().to_lowercase()
    }
}

impl ToString for Key {
    fn to_string(&self) -> String {
        match self {
            Key::W => "w",
            Key::E => "e",
            Key::S => "s",
            Key::D => "d",
        }
        .into()
    }
}

#[derive(Clone)]
pub struct Control {
    pub key:  Key,
    pub cell: Cell,
}

pub struct Controls {
    controls: Vec<Control>,
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            controls: vec![
                Control {
                    key:  Key::W,
                    cell: Cell::TopLeft,
                },
                Control {
                    key:  Key::E,
                    cell: Cell::TopRight,
                },
                Control {
                    key:  Key::S,
                    cell: Cell::BotLeft,
                },
                Control {
                    key:  Key::D,
                    cell: Cell::BotRight,
                },
            ],
        }
    }

    pub fn get_key_by_cell(&self, cell: &Cell) -> Option<Key> {
        self.controls
            .iter()
            .find(|control| &control.cell == cell)
            .map(|control| control.key.clone())
    }

    pub fn get_cell_by_key(&self, key: &Key) -> Option<Cell> {
        self.controls
            .iter()
            .find(|control| &control.key == key)
            .map(|control| control.cell)
    }

    pub fn iter(&self) -> ControlsIter {
        ControlsIter::new(self.controls.clone())
    }
}

use std::iter::Iterator;

pub struct ControlsIter {
    controls: Vec<Control>,
    index:    usize,
}

impl ControlsIter {
    pub fn new(controls: Vec<Control>) -> Self {
        Self { controls, index: 0 }
    }
}

impl Iterator for ControlsIter {
    type Item = Control;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(control) = self.controls.get(self.index) {
            self.index += 1;
            Some(control.clone())
        } else {
            None
        }
    }
}
