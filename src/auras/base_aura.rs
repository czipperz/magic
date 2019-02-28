use crate::bundle::*;
use crate::card::*;
use crate::location::Location;
use crate::player::PlayerNumber;
use crate::state::State;
use crate::triggers::*;
use std::sync::{Arc, Mutex};

pub fn creature_aura(
    triggers: Triggers,
    predicate: fn(&Card, PlayerNumber, Location) -> bool,
) -> Triggers {
    triggers.with_cast_triggers(CreatureAuraCastTrigger { predicate })
}

struct CreatureAuraCastTrigger {
    predicate: fn(&Card, PlayerNumber, Location) -> bool,
}

impl Trigger for CreatureAuraCastTrigger {
    fn can_execute(
        &self,
        state: &State,
        bundle: &Bundle,
        card: Arc<Mutex<Card>>,
        controller: PlayerNumber,
        location: Location,
    ) -> bool {
        state.is_any_card_targetable_by(controller, self.predicate)
    }

    fn try_execute(
        &self,
        state: &State,
        bundle: &mut Bundle,
        card: Arc<Mutex<Card>>,
        controller: PlayerNumber,
        location: Location,
    ) -> bool {
        match state.select_target_card(controller, self.predicate) {
            Some(target_card) => {
                bundle
                    .map
                    .insert("target_card", BundleItem::Card(target_card));
                true
            }
            None => false,
        }
    }

    fn on_execute(
        &self,
        state: &mut State,
        bundle: &mut Bundle,
        card: Arc<Mutex<Card>>,
        controller: PlayerNumber,
        location: Location,
    ) -> bool {
        let target_card = bundle.map["target_card"].unwrap_card();
        let mut target_card = target_card.lock().unwrap();
        if state.is_target_card_valid(&*target_card, controller, self.predicate) {
            target_card.add_aura(card);
            true
        } else {
            false
        }
    }
}
