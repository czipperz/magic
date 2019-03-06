use crate::mana::Color;

#[derive(Clone, Debug, Default)]
pub struct ManaPool {
    pub blue: usize,
    pub white: usize,
    pub green: usize,
    pub red: usize,
    pub black: usize,
    pub generic: usize,
}

impl ManaPool {
    pub fn new(&self) -> Self {
        Self::default()
    }

    pub fn converted(&self) -> usize {
        self.blue + self.white + self.green + self.red + self.black
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
