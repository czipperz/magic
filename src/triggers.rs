use crate::bundle::*;
use crate::card::Card;
use crate::player::PlayerNumber;
use crate::state::State;
use std::fmt;
use std::sync::{Arc, Mutex};

/// This structure represents the triggers a `Card` has.
///
/// The `can_*` functions allow for the user interface to understand
/// if the action can be performed.  The `try_*` functions allow for
/// user interaction that could be canceled.  The `on_*` functions are
/// responses to the successful application of the action.
///
/// The `on_*` functions *must check that the card can still be cast*.
/// Say you are making Lightning Bolt.  It reads "Lightning Bolt deals
/// 3 damage to target creature or player".  `can_cast` must be
/// overridden to test if there is at least one creature or player
/// that can be targeted.  `try_cast` must be overridden to have the
/// user pick the creature or player.  What happens if during the
/// resolve responses step the target becomes invalid (for example,
/// the target becomes hexproof)?  Then `on_cast` must return false.
///
/// # Examples
///
/// Say we try to play a spell from our hand.  This will invoke:
///
/// ```ignore
/// try_cast  --true-->  (responses)  ---->  on_cast  --true-->  (spell resolves)
///           --false->  (abort)                      --false->  (abort)
/// ```
///
/// This allows for spells that have requirements to be cast to ensure
/// they are fulfilled.
#[derive(Clone, Default)]
pub struct Triggers {
    pub cast: Option<Arc<Trigger>>,
}

pub trait Trigger {
    fn can_execute(&self, state: &State, bundle: &Bundle, card: Arc<Mutex<Card>>) -> bool;

    fn try_execute(&self, state: &mut State, bundle: &mut Bundle, card: Arc<Mutex<Card>>) -> bool;

    fn on_execute(&self, state: &mut State, bundle: &mut Bundle, card: Arc<Mutex<Card>>) -> bool;
}

impl Triggers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cast_triggers(mut self, cast_triggers: impl Trigger + 'static) -> Self {
        self.cast = Some(Arc::new(cast_triggers));
        self
    }
}

impl fmt::Debug for Triggers {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Triggers {{ .. }}")
    }
}

pub struct TriggerTargettingPlayer<C> {
    callback: C,
}

impl<C: Fn(&mut State, PlayerNumber) -> bool> TriggerTargettingPlayer<C> {
    pub fn new(callback: C) -> Self {
        TriggerTargettingPlayer { callback }
    }
}

impl<C: Fn(&mut State, PlayerNumber) -> bool> Trigger for TriggerTargettingPlayer<C> {
    fn can_execute(&self, state: &State, bundle: &Bundle, card: Arc<Mutex<Card>>) -> bool {
        state.is_any_player_targetable_by(card.lock().unwrap().controller())
    }

    fn try_execute(&self, state: &mut State, bundle: &mut Bundle, card: Arc<Mutex<Card>>) -> bool {
        match state.select_target_player(card.lock().unwrap().controller()) {
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
        if state.is_target_player_valid(target_player, card.lock().unwrap().controller()) {
            (self.callback)(state, target_player)
        } else {
            false
        }
    }
}

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
