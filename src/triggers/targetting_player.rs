use super::{Bundle, BundleItem, Trigger};
use crate::card::Card;
use crate::player::PlayerNumber;
use crate::source::Source;
use crate::state::State;
use std::sync::{Arc, Mutex};

pub struct TriggerTargettingPlayer<C> {
    callback: C,
}

impl<C: Fn(&mut State, PlayerNumber) -> bool> TriggerTargettingPlayer<C> {
    pub fn new(callback: C) -> Self {
        TriggerTargettingPlayer { callback }
    }
}

impl<C: Fn(&mut State, PlayerNumber) -> bool> Trigger for TriggerTargettingPlayer<C> {
    fn can_execute(&self, state: &State, _: &Bundle, card: Arc<Mutex<Card>>) -> bool {
        state.is_any_player_targetable_by(&Source::from_card(state, card))
    }

    fn try_execute(&self, state: &mut State, bundle: &mut Bundle, card: Arc<Mutex<Card>>) -> bool {
        match state.select_target_player(&Source::from_card(state, card)) {
            Some(target_player) => {
                bundle
                    .map
                    .insert("target_player", BundleItem::Player(target_player));
                true
            }
            None => false,
        }
    }

    fn on_execute(&self, state: &mut State, bundle: &mut Bundle, card: Arc<Mutex<Card>>) -> bool {
        let target_player = bundle.map["target_player"].unwrap_player();
        if state
            .player(target_player)
            .lock()
            .unwrap()
            .is_valid_target(state, &Source::from_card(state, card))
        {
            (self.callback)(state, target_player)
        } else {
            false
        }
    }
}
