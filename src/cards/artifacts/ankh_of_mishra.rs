use crate::card::*;
use crate::mana::ManaCost;
use crate::player::PlayerNumber;
use crate::source::Source;
use crate::triggers::*;
use std::sync::{Arc, Mutex};

pub fn ankh_of_mishra(player: PlayerNumber) -> Arc<Mutex<Card>> {
    let ankh = Arc::new(Mutex::new(Card::new(
        "Ankh of Mishra".to_string(),
        player,
        ManaCost::new().with_generic(2),
        vec![Type::Artifact],
    )));
    let ankh_cloned = ankh.clone();
    ankh.lock().unwrap().base_triggers_mut().set(
        TriggerType::PermanentEntersTheBattlefield,
        TriggerOnExecute::new(move |state, card| {
            let card_state = card.lock().unwrap().state(state);
            if card_state.types.contains(&Type::Land) {
                state
                    .player(card_state.controller)
                    .lock()
                    .unwrap()
                    .take_damage(&Source::from_card(state, ankh_cloned.clone()), 2);
            }
            true
        }),
    );
    ankh
}
