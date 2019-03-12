use crate::mana::ManaPool;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default)]
pub struct ManaCost {
    pub pool: ManaPool,
}

impl ManaCost {
    pub fn new() -> Self {
        Self::default()
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
    pub fn with_generic(mut self, generic: usize) -> Self {
        self.generic = generic;
        self
    }
}

impl Deref for ManaCost {
    type Target = ManaPool;

    fn deref(&self) -> &ManaPool {
        &self.pool
    }
}

impl DerefMut for ManaCost {
    fn deref_mut(&mut self) -> &mut ManaPool {
        &mut self.pool
    }
}
