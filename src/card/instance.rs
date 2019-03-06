use crate::card::{Card, Zone};
use crate::player::PlayerNumber;
use crate::permanent::Permanent;
use std::sync::{Mutex, Weak};

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
