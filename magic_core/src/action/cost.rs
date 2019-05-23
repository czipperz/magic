use crate::count::Count;
use crate::instance::InstanceID;
use crate::mana::{ManaCost, ManaPayment};
use crate::state::State;
use by_address::ByAddress;
use std::fmt;
use std::sync::Arc;

/// The payment supplied to satisfy the `Cost`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Payment {
    Tap,
    Mana(ManaPayment),
    Sacrifice(Vec<InstanceID>),
}

/// The cost needed to perform an `Action`.
#[derive(Clone, Eq, PartialEq)]
pub enum Cost {
    Tap,
    Mana(ManaCost),
    Sacrifice(Count, ByAddress<Arc<Fn(&State, InstanceID) -> bool>>),
}

impl fmt::Debug for Cost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cost::")?;
        match self {
            Cost::Tap => write!(f, "Tap"),
            Cost::Mana(cost) => write!(f, "Mana({:?})", cost),
            Cost::Sacrifice(count, _) => write!(f, "Sacrifice({:?}, {:?})", count, ".."),
        }
    }
}
