use crate::mana::Color;

#[derive(Clone, Debug, Default)]
pub struct ManaCost {
    pub blue: usize,
    pub white: usize,
    pub green: usize,
    pub red: usize,
    pub black: usize,
    pub generic: usize,
}

impl ManaCost {
    pub fn new() -> Self {
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
    pub fn with_generic(mut self, generic: usize) -> Self {
        self.generic = generic;
        self
    }
}
