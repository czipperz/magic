use crate::bundle::Bundle;
use crate::card::Card;
use crate::location::Location;
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
#[derive(Clone)]
pub struct Triggers {
    pub cast_triggers: Option<Arc<CastTriggers>>,
}

pub trait CastTriggers {
    fn can_cast(
        &self,
        state: &State,
        bundle: &Bundle,
        card: Arc<Mutex<Card>>,
        controller: PlayerNumber,
        location: Location,
    ) -> bool;

    fn try_cast(
        &self,
        state: &State,
        bundle: &mut Bundle,
        card: Arc<Mutex<Card>>,
        controller: PlayerNumber,
        location: Location,
    ) -> bool;

    fn on_cast(
        &self,
        state: &mut State,
        bundle: &mut Bundle,
        card: Arc<Mutex<Card>>,
        controller: PlayerNumber,
        location: Location,
    ) -> bool;
}

impl Triggers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cast_triggers(mut self, cast_triggers: impl CastTriggers + 'static) -> Self {
        self.cast_triggers = Some(Arc::new(cast_triggers));
        self
    }
}

impl Default for Triggers {
    fn default() -> Self {
        Triggers {
            cast_triggers: None,
        }
    }
}

impl fmt::Debug for Triggers {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Triggers {{ .. }}")
    }
}
