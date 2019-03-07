use crate::card::{Instance, Payment, Zone};
use crate::player::PlayerNumber;
use crate::source::Source;
use crate::spell::{Ability, Spell, Target};
use std::sync::{Arc, Mutex};

/// An `Event` is an item that can cause triggers.
///
/// All variants run triggers.  The `Cast` variant run triggers then
/// allow users to respond.  After triggers have ran and they are put
/// on the stack, users can respond to the triggers.
pub struct Event {
    pub source: Source,
    pub v: EventV,
}

impl Event {
    pub fn cast(
        card: Arc<Mutex<Instance>>,
        mandatory_payments: Vec<Payment>,
        optional_payments: Vec<Option<Payment>>,
        targets: Vec<Target>,
    ) -> Self {
        Event {
            source: card.clone().into(),
            v: EventV::Cast(Spell::new(
                card,
                mandatory_payments,
                optional_payments,
                targets,
            )),
        }
    }
}

pub enum EventV {
    Cast(Spell),
    Activated(Ability),
    HealthOffset(PlayerNumber, isize),
    CardMoved(Arc<Mutex<Instance>>, PlayerNumber, Zone, PlayerNumber, Zone),
    CompositeEvent(Vec<Event>),
}
