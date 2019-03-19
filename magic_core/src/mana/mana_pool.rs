use crate::mana::Color;
use std::ops::{Add, AddAssign};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ManaPool {
    pub blue: usize,
    pub white: usize,
    pub green: usize,
    pub red: usize,
    pub black: usize,
    pub colorless: usize,
}

impl ManaPool {
    pub fn new(&self) -> Self {
        Self::default()
    }

    pub fn converted(&self) -> usize {
        self.blue + self.white + self.green + self.red + self.black + self.colorless
    }

    pub fn colors(&self) -> Vec<Color> {
        let mut colors = Vec::new();
        if self.blue != 0 {
            colors.push(Color::Blue);
        }
        if self.white != 0 {
            colors.push(Color::White);
        }
        if self.green != 0 {
            colors.push(Color::Green);
        }
        if self.red != 0 {
            colors.push(Color::Red);
        }
        if self.black != 0 {
            colors.push(Color::Black);
        }
        colors
    }
}

impl Add<&Self> for ManaPool {
    type Output = ManaPool;

    fn add(mut self, other: &ManaPool) -> ManaPool {
        self += other;
        self
    }
}

impl AddAssign<&Self> for ManaPool {
    fn add_assign(&mut self, other: &ManaPool) {
        self.blue += other.blue;
        self.white += other.white;
        self.green += other.green;
        self.red += other.red;
        self.black += other.black;
        self.colorless += other.colorless;
    }
}
