use crate::card::{Instance, Payment, Zone};
use crate::player::PlayerNumber;
use crate::source::Source;
use crate::spell::{Spell, Target};
use std::sync::{Arc, Mutex};

/// An `Event` is an item on the stack.
pub struct Event {
    pub source: Source,
    pub v: EventV,
}

impl Event {
    pub fn cast(card: Arc<Mutex<Instance>>, payment: Payment, targets: Vec<Target>) -> Self {
        Event {
            source: card.clone().into(),
            v: EventV::Cast(Spell::new(card, payment, targets)),
        }
    }
}

pub enum EventV {
    Cast(Spell),
    OffsetHealth(PlayerNumber, isize),
    MoveCard(Arc<Mutex<Instance>>, PlayerNumber, Zone),
    CompositeEvent(Vec<EventV>),
}
