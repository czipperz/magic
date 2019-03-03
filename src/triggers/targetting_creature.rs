use super::{Bundle, BundleItem, Trigger};
use crate::card::Card;
use crate::source::Source;
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
    fn can_execute(&self, state: &State, _: &Bundle, card: Arc<Mutex<Card>>) -> bool {
        state.is_any_permanent_targetable_by(&Source::from_card(state, card), &self.predicate)
    }

    fn try_execute(&self, state: &mut State, bundle: &mut Bundle, card: Arc<Mutex<Card>>) -> bool {
        match state.select_target_card(&Source::from_card(state, card), &self.predicate) {
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
            &Source::from_card(state, card.clone()),
            &self.predicate,
        ) {
            (self.callback)(state, card, target_card)
        } else {
            false
        }
    }
}
