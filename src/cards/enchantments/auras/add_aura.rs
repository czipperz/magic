use crate::card::*;
use crate::state::State;
use std::sync::{Arc, Mutex};

pub fn add_aura(
    decoration: impl Clone + Fn(&State, &Card, &mut CardState) + 'static,
) -> impl Fn(&mut State, Arc<Mutex<Card>>, Arc<Mutex<Card>>) -> bool {
    move |_, card, target_card| {
        target_card
            .lock()
            .unwrap()
            .add_aura(card, decoration.clone());
        true
    }
}
