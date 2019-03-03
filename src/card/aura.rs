use crate::card::{Card, CardState};
use crate::state::State;
use std::fmt;
use std::sync::{Arc, Mutex};

pub struct Aura {
    pub card: Arc<Mutex<Card>>,
    pub decoration: Box<Fn(&State, &Card, &mut CardState)>,
}

impl Aura {
    pub fn decorate_card_state(&self, state: &State, card: &Card, card_state: &mut CardState) {
        (self.decoration)(state, card, card_state)
    }
}

impl fmt::Debug for Aura {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.card.lock().unwrap().name())
    }
}
