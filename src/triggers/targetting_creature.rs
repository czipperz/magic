use super::Trigger;
use crate::bundle::*;
use crate::card::Card;
use crate::state::State;
use std::sync::{Arc, Mutex};

pub struct TriggerTargettingCreature<P, C> {
    predicate: P,
    callback: C,
}

impl<P, C> TriggerTargettingCreature<P, C>
where
    P: Fn(&State, &Card) -> bool,
    C: Fn(
        &mut State,
        /* card with the trigger */ Arc<Mutex<Card>>,
        /* target */ Arc<Mutex<Card>>,
    ) -> bool,
{
    pub fn new(predicate: P, callback: C) -> Self {
        TriggerTargettingCreature {
            predicate,
            callback,
        }
    }
}

impl<P, C> Trigger for TriggerTargettingCreature<P, C>
where
    P: Fn(&State, &Card) -> bool,
    C: Fn(&mut State, Arc<Mutex<Card>>, Arc<Mutex<Card>>) -> bool,
{
    fn can_execute(&self, state: &State, bundle: &Bundle, card: Arc<Mutex<Card>>) -> bool {
        state.is_any_card_targetable_by(card.lock().unwrap().controller(), &self.predicate)
    }

    fn try_execute(&self, state: &mut State, bundle: &mut Bundle, card: Arc<Mutex<Card>>) -> bool {
        match state.select_target_card(card.lock().unwrap().controller(), &self.predicate) {
            Some(target_card) => {
                bundle
                    .map
                    .insert("target_card", BundleItem::Card(target_card));
                true
            }
            None => false,
        }
    }

    fn on_execute(&self, state: &mut State, bundle: &mut Bundle, card: Arc<Mutex<Card>>) -> bool {
        let target_card = bundle.map["target_card"].unwrap_card();
        if target_card.lock().unwrap().is_valid_target(
            state,
            card.lock().unwrap().controller(),
            &self.predicate,
        ) {
            (self.callback)(state, card, target_card)
        } else {
            false
        }
    }
}
