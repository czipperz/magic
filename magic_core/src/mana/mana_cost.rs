use crate::mana::ManaPool;
use std::ops::{Add, AddAssign, Deref, DerefMut};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ManaCost {
    pub pool: ManaPool,
    pub generic: usize,
}

impl ManaCost {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn converted(&self) -> usize {
        self.pool.converted() + self.generic
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

impl Add<&Self> for ManaCost {
    type Output = ManaCost;

    fn add(mut self, other: &ManaCost) -> ManaCost {
        self += other;
        self
    }
}

impl AddAssign<&Self> for ManaCost {
    fn add_assign(&mut self, other: &ManaCost) {
        self.pool += &other.pool;
        self.generic += other.generic;
    }
}
