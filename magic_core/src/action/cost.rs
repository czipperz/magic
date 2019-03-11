use crate::count::Count;
use crate::instance::InstanceNumber;
use crate::mana::{ManaCost, ManaPool};
use crate::permanent::PermanentNumber;
use crate::state::State;
use std::sync::Arc;

/// The payment supplied to satisfy the `Cost`.
pub enum Payment {
    Mana(ManaPool),
    Sacrifice(Vec<InstanceNumber>),
}

/// The cost needed to perform an `Action`.
#[derive(Clone)]
pub enum Cost {
    Mana(ManaCost),
    Sacrifice(Count, Arc<Fn(&State, PermanentNumber) -> bool>),
}
