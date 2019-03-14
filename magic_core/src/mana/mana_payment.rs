use crate::mana::ManaPool;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
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
