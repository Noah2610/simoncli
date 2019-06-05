use rand::{thread_rng, Rng};

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    TopLeft,
    TopRight,
    BotLeft,
    BotRight,
}

impl Cell {
    pub fn random() -> Self {
        const LENGTH: usize = 4;

        let mut rng = thread_rng();
        let random = rng.gen_range(0, LENGTH);

        match random {
            0 => Cell::TopLeft,
            1 => Cell::TopRight,
            2 => Cell::BotLeft,
            3 => Cell::BotRight,
            n => panic!("Invalid random index: {}", n),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Cell::TopLeft => "TopLeft",
            Cell::TopRight => "TopRight",
            Cell::BotLeft => "BotLeft",
            Cell::BotRight => "BotRight",
        }
    }
}
