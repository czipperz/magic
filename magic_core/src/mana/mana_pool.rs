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
    pub fn new() -> Self {
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

    pub fn with_blue(mut self, blue: usize) -> Self {
        self.blue = blue;
        self
    }
    pub fn with_white(mut self, white: usize) -> Self {
        self.white = white;
        self
    }
    pub fn with_green(mut self, green: usize) -> Self {
        self.green = green;
        self
    }
    pub fn with_red(mut self, red: usize) -> Self {
        self.red = red;
        self
    }
    pub fn with_black(mut self, black: usize) -> Self {
        self.black = black;
        self
    }
    pub fn with_colorless(mut self, colorless: usize) -> Self {
        self.colorless = colorless;
        self
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors_empty() {
        assert_eq!(ManaPool::default().colors(), vec![]);
    }

    #[test]
    fn test_colors_multi() {
        let mut pool = ManaPool::default();
        pool.blue = 1;
        pool.red = 1;
        assert_eq!(pool.colors(), vec![Color::Blue, Color::Red]);
    }

    #[test]
    fn test_colors_colorless() {
        let mut pool = ManaPool::default();
        pool.blue = 1;
        pool.colorless = 1;
        assert_eq!(pool.colors(), vec![Color::Blue]);
    }

    #[test]
    fn test_converted_include_colorless() {
        let mut pool = ManaPool::default();
        pool.black = 10;
        pool.colorless = 2;
        assert_eq!(pool.converted(), 12);
    }
}
