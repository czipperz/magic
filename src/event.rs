use crate::card::{Instance, Zone};
use crate::player::PlayerNumber;
use crate::source::Source;
use crate::spell::Spell;
use std::sync::{Arc, Mutex};

/// An `Event` is an item on the stack.
pub struct Event {
    pub source: Source,
    pub v: EventV,
}

pub enum EventV {
    Cast(Spell),
    OffsetHealth(PlayerNumber, isize),
    MoveCard(Arc<Mutex<Instance>>, PlayerNumber, Zone),
    CompositeEvent(Vec<EventV>),
}
