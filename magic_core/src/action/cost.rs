use crate::count::Count;
use crate::instance::InstanceNumber;
use crate::mana::{ManaCost, ManaPayment};
use crate::permanent::PermanentNumber;
use crate::state::State;
use by_address::ByAddress;
use std::fmt;
use std::sync::Arc;

/// The payment supplied to satisfy the `Cost`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Payment {
    Mana(ManaPayment),
    Sacrifice(Vec<InstanceNumber>),
}

/// The cost needed to perform an `Action`.
#[derive(Clone, Eq, PartialEq)]
pub enum Cost {
    Mana(ManaCost),
    Sacrifice(Count, ByAddress<Arc<Fn(&State, PermanentNumber) -> bool>>),
}

impl fmt::Debug for Cost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cost::")?;
        match self {
            Cost::Mana(cost) => write!(f, "Mana({:?})", cost),
            Cost::Sacrifice(count, _) => write!(f, "Sacrifice({:?}, {:?})", count, ".."),
        }
    }
}
