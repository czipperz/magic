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
pub struct SourcedEvent {
    pub source: Source,
    pub event: Event,
}

impl SourcedEvent {
    pub fn cast(
        card: Arc<Mutex<Instance>>,
        mandatory_payments: Vec<Payment>,
        optional_payments: Vec<Option<Payment>>,
        targets: Vec<Target>,
    ) -> Self {
        SourcedEvent {
            source: card.clone().into(),
            event: Event::Cast(Spell::new(
                card,
                mandatory_payments,
                optional_payments,
                targets,
            )),
        }
    }
}

pub enum Event {
    PlayLand(Arc<Mutex<Instance>>),
    Cast(Spell),
    Activated(Ability),
    DamageTaken(PlayerNumber, isize),
    CardsDrawn(PlayerNumber, usize),
    CardMoved(Arc<Mutex<Instance>>, PlayerNumber, Zone, PlayerNumber, Zone),
    CompositeEvent(Vec<Event>),
}

impl Event {
    pub fn with_source(self, source: Source) -> SourcedEvent {
        SourcedEvent {
            source,
            event: self,
        }
    }

    pub fn move_to_zone(instance: Arc<Mutex<Instance>>, zone: Zone) -> Event {
        let (controller, old_zone) = {
            let instance = instance.lock().unwrap();
            (instance.controller(), instance.zone())
        };
        Event::CardMoved(instance, controller, old_zone, controller, zone)
    }
}
