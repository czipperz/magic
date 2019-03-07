use crate::card::{Card, Zone};
use crate::event::EventV;
use crate::permanent::Permanent;
use crate::player::PlayerNumber;
use std::ops::Deref;
use std::sync::{Arc, Mutex, Weak};

/// An instance of a particular `Card`.  This encapsulates the
/// location of a card.
#[derive(Debug)]
pub struct Instance {
    card: Card,
    permanent: Weak<Mutex<Permanent>>,
    controller: PlayerNumber,
    zone: Zone,
}

impl Instance {
    pub fn new(card: Card, controller: PlayerNumber, zone: Zone) -> Self {
        Instance {
            card,
            permanent: Weak::new(),
            controller,
            zone,
        }
    }

    pub fn card(&self) -> &Card {
        &self.card
    }
    pub fn permanent(&self) -> &Weak<Mutex<Permanent>> {
        &self.permanent
    }
    pub fn controller(&self) -> PlayerNumber {
        self.controller
    }
    pub fn zone(&self) -> Zone {
        self.zone
    }
}

impl Instance {
    pub fn move_to_zone(instance: Arc<Mutex<Instance>>, zone: Zone) -> EventV {
        let (controller, old_zone) = {
            let mut instance = instance.lock().unwrap();
            let old_zone = instance.zone;
            instance.zone = zone;
            (instance.controller, old_zone)
        };
        EventV::CardMoved(instance, controller, old_zone, controller, zone)
    }
}

impl Deref for Instance {
    type Target = Card;

    fn deref(&self) -> &Card {
        &self.card
    }
}
