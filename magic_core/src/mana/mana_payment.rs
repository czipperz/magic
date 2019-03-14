use crate::mana::ManaPool;
use std::ops::{Add, AddAssign, Deref, DerefMut};

#[derive(Clone, Debug, Default)]
pub struct ManaPayment {
    pub pool: ManaPool,
    pub generic: usize,
}

impl Deref for ManaPayment {
    type Target = ManaPool;

    fn deref(&self) -> &ManaPool {
        &self.pool
    }
}

impl DerefMut for ManaPayment {
    fn deref_mut(&mut self) -> &mut ManaPool {
        &mut self.pool
    }
}

impl Add<&Self> for ManaPayment {
    type Output = ManaPayment;

    fn add(mut self, other: &ManaPayment) -> ManaPayment {
        self += other;
        self
    }
}

impl AddAssign<&Self> for ManaPayment {
    fn add_assign(&mut self, other: &ManaPayment) {
        self.pool += &other.pool;
        self.generic += other.generic;
    }
}
